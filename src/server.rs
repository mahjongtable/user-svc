use arc_swap::ArcSwap;
use sqlx::{Error, MySql};
use tonic::{Request, Response, Status, transport::Server};
use user_svc::{
    db::{self, DbUserRepository, repository::UserRepository},
    pb::user::{
        CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse,
        GetUserProfileRequest, GetUserProfileResponse, GetUserRequest, GetUserResponse,
        user_server::{User, UserServer},
    },
    settings::{AppSettings, init_config},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_settings = init_config("settings.toml")?;

    println!("The database is connecting...");
    let pool = db::connect::<MySql>(&app_settings.load().database).await?;
    println!("The database has connected successfully");

    let db_user_repo = DbUserRepository { pool };

    Server::builder()
        .add_service(UserServer::new(UserService::new(
            app_settings,
            db_user_repo,
        )))
        .serve("[::1]:50051".parse()?)
        .await?;

    Ok(())
}

struct UserService<R>
where
    R: UserRepository,
{
    app_settings: ArcSwap<AppSettings>,
    repo: R,
}

impl<R> UserService<R>
where
    R: UserRepository,
{
    fn new(app_settings: ArcSwap<AppSettings>, repo: R) -> Self {
        Self { app_settings, repo }
    }
}

#[tonic::async_trait]
impl<R: UserRepository + 'static> User for UserService<R> {
    async fn create_user(
        &self,
        req: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let new_id = self
            .repo
            .create_user(req.into_inner().into())
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        Ok(Response::new(CreateUserResponse { uid: new_id as i64 }))
    }

    async fn delete_user(
        &self,
        _request: Request<DeleteUserRequest>,
    ) -> Result<Response<DeleteUserResponse>, Status> {
        Ok(Response::new(DeleteUserResponse {}))
    }

    async fn get_user_profile(
        &self,
        req: Request<GetUserProfileRequest>,
    ) -> Result<Response<GetUserProfileResponse>, Status> {
        let user = self
            .repo
            .get_user(req.get_ref().uid as u64)
            .await
            .map_err(|err| Status::internal(err.to_string()))?;

        println!("{:#?}", user);

        Ok(Response::new(GetUserProfileResponse {
            username: user.username.unwrap_or_default(),
            gender: user.gender.unwrap_or(1) as i32,
            avatar_url: user.avatar_url,
        }))
    }

    async fn get_user(
        &self,
        req: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let user_id = req.into_inner().uid;

        let user = self
            .repo
            .get_user(user_id as u64)
            .await
            .map_err(|err| match err {
                Error::RowNotFound => Status::not_found("The user doesn't exist or has been deleted."),
                _ => Status::internal("An error occurred while accessing the database."),
            })?;

        Ok(Response::new(user.into()))
    }
}
