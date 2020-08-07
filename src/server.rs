pub mod pb {
  tonic::include_proto!("hello");
}
use pb::say_server::{Say, SayServer};
use pb::{SayRequest, SayResponse};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
pub struct MySay {}
#[tonic::async_trait]
impl Say for MySay {
  // defining return stream
  type BidirectionalStream = mpsc::Receiver<Result<SayResponse, Status>>;
  async fn bidirectional(
    &self,
    request: Request<tonic::Streaming<SayRequest>>,
  ) -> Result<Response<Self::BidirectionalStream>, Status> {
    // converting request in stream
    let mut streamer = request.into_inner();
    // creating queue
    let (mut tx, rx) = mpsc::channel(4);
    tokio::spawn(async move {
      // listening on request stream
      while let Some(req) = streamer.message().await.unwrap() {
        // sending data as soon it is available
        tx.send(Ok(SayResponse {
          message: format!("hello {}", req.name),
        }))
        .await.ok();
      }
    });
    // returning stream as receiver
    Ok(Response::new(rx))
  }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let addr = "[::1]:50051".parse().unwrap();
  let say = MySay::default();
  println!("Server listening on {}", addr);
  Server::builder()
    .add_service(SayServer::new(say))
    .serve(addr)
    .await?;
  Ok(())
}
