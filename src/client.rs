pub mod pb {
  tonic::include_proto!("hello");
}
use futures::stream::iter;
use pb::say_client::SayClient;
use pb::SayRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
    .connect()
    .await?;
  let mut client = SayClient::new(channel);
  // creating a client
  let request = tonic::Request::new(iter(vec![
    SayRequest {
      name: String::from("anshul"),
    },
    SayRequest {
      name: String::from("rahul"),
    },
    SayRequest {
      name: String::from("vijay"),
    },
    SayRequest {
      name: String::from("alec"),
    },
  ]));
  // calling rpc
  let mut response = client.bidirectional(request).await?.into_inner();
  // listening on the response stream
  while let Some(res) = response.message().await? {
    println!("NOTE = {:?}", res);
  }
  Ok(())
}
