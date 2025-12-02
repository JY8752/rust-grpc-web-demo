use proto::user::v1::{
    CreateUserRequest, CreateUserResponse, GetUserRequest, GetUserResponse, User,
    user_service_server::{UserService, UserServiceServer},
};
use tonic::{Request, Response, Status, transport::Server};

pub mod proto {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = include_bytes!("../descriptor.binpb");
    pub mod user {
        pub mod v1 {
            tonic::include_proto!("user.v1");
        }
    }
}

#[derive(Debug, Default)]
struct MyUserService {}

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let request = request.into_inner();
        let user = request.user;

        println!("Creating user: {:?}", user);

        Ok(Response::new(CreateUserResponse {}))
    }

    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserResponse>, Status> {
        let request = request.into_inner();
        let id = request.id;

        Ok(Response::new(GetUserResponse {
            user: Some(User {
                id,
                name: "John Doe".to_string(),
            }),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1()
        .unwrap();

    let addr = "[::1]:50051".parse().unwrap();
    let service = MyUserService::default();

    println!("UserService listening on {addr}");

    Server::builder()
        .add_service(reflection_service)
        .add_service(UserServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
