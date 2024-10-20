use hello::greeter_client::GreeterClient;
use hello::GreetRequest;

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(GreetRequest {
        name: "Tonic".into(),
    });

    let response = client.greet(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
