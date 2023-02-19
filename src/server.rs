use tonic::{transport::Server, Request, Response, Status};

use jaeger_storage_v1::span_writer_plugin_server::{SpanWriterPlugin, SpanWriterPluginServer};
use jaeger_storage_v1::{WriteSpanRequest, WriteSpanResponse, CloseWriterRequest, CloseWriterResponse, span_reader_plugin_server};

pub mod jaeger_storage_v1 {
    tonic::include_proto!("jaeger"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct CSpanWriterPlugin {}

#[tonic::async_trait]
impl SpanWriterPlugin for CSpanWriterPlugin {
    async fn write_span(
        &self,
        request: Request<WriteSpanRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<WriteSpanResponse>, Status> { // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = jaeger_storage_v1::WriteSpanResponse {
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }

    async fn close(
        &self,
        request: Request<CloseWriterRequest>,
    ) -> Result<Response<CloseWriterResponse>, Status> {
        println!("Got a request: {:?}", request);

        let reply = jaeger_storage_v1::CloseWriterResponse {
        };

        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let span_writer_plugin_service = CSpanWriterPlugin::default();

    Server::builder()
        .add_service(SpanWriterPluginServer::new(span_writer_plugin_service))
        .serve(addr)
        .await?;

    Ok(())
}
