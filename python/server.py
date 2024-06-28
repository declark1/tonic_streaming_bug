from concurrent.futures import ThreadPoolExecutor

import grpc
import echo_pb2 as pb
import echo_pb2_grpc as pb_grpc

class EchoService(pb_grpc.EchoServicer):
    def BidirectionalStreamingEcho(
        self, request_iterator, context: grpc.ServicerContext
    ) -> pb.EchoResponse:
        # tonic client hangs without sending this initial metadata
        # comment out to reproduce
        context.send_initial_metadata((("x-grpc-test-echo-initial", "")))
        for request in request_iterator:
            print(request)
            yield pb.EchoResponse(message=request.message)

def serve() -> None:
    server = grpc.server(ThreadPoolExecutor(max_workers=4))
    pb_grpc.add_EchoServicer_to_server(EchoService(), server)
    listen_addr = "[::]:50051"
    server.add_insecure_port(listen_addr)
    print("Starting server on %s", listen_addr)
    server.start()
    server.wait_for_termination()

if __name__ == "__main__":
    serve()