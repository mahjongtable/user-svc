use tonic::{Request, Response, Status, transport::Server};
use user_svc::pb::user::{
    CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse,
    GetUserProfileRequest, GetUserProfileResponse,
    user_server::{User, UserServer},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Server::builder()
        .add_service(UserServer::new(UserService::new()))
        .serve("[::1]:50051".parse()?)
        .await?;

    Ok(())
}

struct UserService;

impl UserService {
    fn new() -> Self {
        Self
    }
}

#[tonic::async_trait]
impl User for UserService {
    async fn create_user(
        &self,
        _request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        Ok(Response::new(CreateUserResponse {
            uid: "1".to_string(),
        }))
    }

    async fn delete_user(
        &self,
        _request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        Ok(Response::new(DeleteUserResponse {}))
    }

    async fn get_user_profile(
        &self,
        _request: Request<GetUserProfileRequest>,
    ) -> Result<Response<GetUserProfileResponse>, Status> {
        Ok(Response::new(GetUserProfileResponse {
            username: "Lucas".to_owned(),
            gender: 1,
            avatar_url: Some(
                "https://avatars.githubusercontent.com/u/53471930?v=4&size=64".to_owned(),
            ),
        }))
    }
}
