use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use prost_types::Timestamp;
/* Import compiled proto */
use proto::auth_service_server::{ AuthService, AuthServiceServer };
use tonic::transport::Server;

mod proto {
  tonic::include_proto!("auth");

  pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("auth_descriptor");
}

#[derive(Debug, Default)]
struct AuthServiceImpl {}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
  async fn login(&self, request: tonic::Request<proto::LoginRequest>) -> Result<tonic::Response<proto::LoginResponse>, tonic::Status> {
    let request = request.into_inner();
    println!("Received request: {:?}", request);

    let response = proto::LoginResponse {
      token: "token".to_string(),
      expires_at: Some(Timestamp::from(std::time::SystemTime::now())),
    };

    Ok(tonic::Response::new(response))
  }

  async fn register(&self, request: tonic::Request<proto::RegisterRequest>) -> Result<tonic::Response<proto::RegisterResponse>, tonic::Status> {
    let request = request.into_inner();
    println!("Received request: {:?}", request);

    let response = proto::RegisterResponse {
      message: "User registered successfully".to_string(),
    };

    Ok(tonic::Response::new(response))
  }

  async fn logout(&self, request: tonic::Request<proto::LogoutRequest>) -> Result<tonic::Response<proto::LogoutResponse>, tonic::Status> {
    let request = request.into_inner();
    println!("Received request: {:?}", request);

    let response = proto::LogoutResponse {
      message: "User logged out successfully".to_string(),
    };

    Ok(tonic::Response::new(response))
  }

  async fn verify_token(&self, request: tonic::Request<proto::VerifyTokenRequest>) -> Result<tonic::Response<proto::VerifyTokenResponse>, tonic::Status> {
    let request = request.into_inner();
    println!("Received request: {:?}", request);

    let response = proto::VerifyTokenResponse {
      message: "Token is valid".to_string(),
    };

    Ok(tonic::Response::new(response))
  }

  async fn refresh_token(&self, request: tonic::Request<proto::RefreshTokenRequest>) -> Result<tonic::Response<proto::RefreshTokenResponse>, tonic::Status> {
    let request = request.into_inner();
    println!("Received request: {:?}", request);

    let response = proto::RefreshTokenResponse {
      token: "new_token".to_string(),
      expires_at: Some(Timestamp::from(std::time::SystemTime::now())),
    };

    Ok(tonic::Response::new(response))
  }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Server running on port 50002");

  let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 50002); 
  let user_service = AuthServiceImpl::default();
  let svc = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
    .build()?;

  Server::builder()
    .accept_http1(true)
    .add_service(svc)
    .add_service(AuthServiceServer::new(user_service))
    .serve(addr)
    .await?;

  Ok(())
}
