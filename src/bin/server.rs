use std::{net::ToSocketAddrs, pin::Pin};

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status, Streaming};

use tonic_streaming_bug::pb::echo::{echo_server, EchoRequest, EchoResponse};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

type ResponseStream = Pin<Box<dyn Stream<Item = Result<EchoResponse, Status>> + Send>>;

#[derive(Debug, Default)]
pub struct EchoServer {}

#[tonic::async_trait]
impl echo_server::Echo for EchoServer {
    type BidirectionalStreamingEchoStream = ResponseStream;

    async fn bidirectional_streaming_echo(
        &self,
        req: Request<Streaming<EchoRequest>>,
    ) -> Result<Response<Self::BidirectionalStreamingEchoStream>, Status> {
        let mut in_stream = req.into_inner();

        let (out_tx, out_rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(result) = in_stream.next().await {
                match result {
                    Ok(v) => out_tx
                        .send(Ok(EchoResponse { message: v.message }))
                        .await
                        .unwrap(),
                    Err(err) => match out_tx.send(Err(err)).await {
                        Ok(_) => (),
                        Err(_err) => break,
                    },
                }
            }
        });
        let out_stream = ReceiverStream::new(out_rx);
        Ok(Response::new(
            Box::pin(out_stream) as Self::BidirectionalStreamingEchoStream
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("DEBUG"));
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
    let server = EchoServer::default();
    Server::builder()
        .add_service(echo_server::EchoServer::new(server))
        .serve("[::1]:50051".to_socket_addrs().unwrap().next().unwrap())
        .await
        .unwrap();
    Ok(())
}
