# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
"""Client and server classes corresponding to protobuf-defined services."""
import grpc

import echo_pb2 as echo__pb2


class EchoStub(object):
    """Echo is the echo service.
    """

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.BidirectionalStreamingEcho = channel.stream_stream(
                '/echo.Echo/BidirectionalStreamingEcho',
                request_serializer=echo__pb2.EchoRequest.SerializeToString,
                response_deserializer=echo__pb2.EchoResponse.FromString,
                )


class EchoServicer(object):
    """Echo is the echo service.
    """

    def BidirectionalStreamingEcho(self, request_iterator, context):
        """// UnaryEcho is unary echo.
        rpc UnaryEcho(EchoRequest) returns (EchoResponse) {}
        // ServerStreamingEcho is server side streaming.
        rpc ServerStreamingEcho(EchoRequest) returns (stream EchoResponse) {}
        // ClientStreamingEcho is client side streaming.
        rpc ClientStreamingEcho(stream EchoRequest) returns (EchoResponse) {}
        BidirectionalStreamingEcho is bidi streaming.
        """
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_EchoServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'BidirectionalStreamingEcho': grpc.stream_stream_rpc_method_handler(
                    servicer.BidirectionalStreamingEcho,
                    request_deserializer=echo__pb2.EchoRequest.FromString,
                    response_serializer=echo__pb2.EchoResponse.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'echo.Echo', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class Echo(object):
    """Echo is the echo service.
    """

    @staticmethod
    def BidirectionalStreamingEcho(request_iterator,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            insecure=False,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.stream_stream(request_iterator, target, '/echo.Echo/BidirectionalStreamingEcho',
            echo__pb2.EchoRequest.SerializeToString,
            echo__pb2.EchoResponse.FromString,
            options, channel_credentials,
            insecure, call_credentials, compression, wait_for_ready, timeout, metadata)
