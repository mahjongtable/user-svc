use arc_swap::ArcSwap;
use sqlx::MySql;
use tonic::{Request, Response, Status, transport::Server};
use user_svc::{
    db::{
        self, DbUserRepository,
        repository::{User as UserModel, UserRepository},
    },
    pb::user::{
        CreateUserRequest, CreateUserResponse, DeleteUserRequest, DeleteUserResponse,
        GetUserProfileRequest, GetUserProfileResponse,
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
        let req_refed = req.get_ref();

        let new_id = self.repo
            .create_user(UserModel {
                id: None,
                username: req_refed.username.clone(),
                gender: req_refed.gender.clone(),
                avatar_url: req_refed.avatar_url.clone(),
                cellphone_number: req_refed.cellphone_number.clone(),
                email: req_refed.email.clone(),
                password: req_refed.password.clone(),
                created_at: None,
                updated_at: None,
                deleted_at: None,
            })
            .await
            .map_err(|err| Status::internal(err.to_string()))?
            ;

        Ok(Response::new(CreateUserResponse {
            uid: new_id as i64, 
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
