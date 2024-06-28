#![allow(dead_code)]
use std::time::Duration;

use futures::{Stream, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use tonic_streaming_bug::pb::echo::{echo_client::EchoClient, EchoRequest};

fn requests_iter() -> impl Stream<Item = EchoRequest> {
    futures::stream::iter(1..10).map(|i| EchoRequest {
        message: format!("msg {:02}", i),
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::try_from_default_env().unwrap_or(EnvFilter::new("DEBUG"));
    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
    let mut client = EchoClient::connect("http://[::1]:50051").await.unwrap();
    let (in_tx, in_rx) = mpsc::channel(32);
    info!("creating out_stream with empty in_stream");
    let mut out_stream = client
        .bidirectional_streaming_echo(tonic::Request::new(ReceiverStream::new(in_rx)))
        .await?
        .into_inner();
    info!("out_stream created");
    info!("sleeping 5 seconds before sending requests to in_stream");
    tokio::time::sleep(Duration::from_secs(5)).await;
    tokio::spawn(async move {
        for i in 1..=10 {
            let req = EchoRequest {
                message: format!("msg {i:02}"),
            };
            println!("send: {req:?}");
            let _ = in_tx.send(req).await;
        }
    });
    while let Some(Ok(resp)) = out_stream.next().await {
        println!("recv: {resp:?}");
    }
    Ok(())
}
