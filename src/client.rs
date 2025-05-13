use tonic::client;
use user_svc::pb::user::{user_client::UserClient, GetUserRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = UserClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(GetUserRequest {
        uid: 22,
    });

    let response = client.get_user(request).await?;
    println!("{:#?}", &response.into_inner());

    Ok(())
}