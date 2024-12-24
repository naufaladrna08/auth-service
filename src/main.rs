use std::{net::{IpAddr, Ipv4Addr, SocketAddr}, path::PathBuf};
use axum::{
  http::header, response::{IntoResponse, Response}, routing::get, Router
};

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
  let grpc_server = grpc_server();
  let http_server = http_server();
  
  tokio::try_join!(grpc_server, http_server)?;
  
  Ok(())
}

async fn grpc_server() -> Result<(), Box<dyn std::error::Error>> {
  let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 50002); 
  let user_service = AuthServiceImpl::default();
  let svc = tonic_reflection::server::Builder::configure()
    .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
    .build()?;

  println!("gRPC server listening on {}", addr);

  Server::builder()
    .accept_http1(true)
    .add_service(svc)
    .add_service(AuthServiceServer::new(user_service))
    .serve(addr)
    .await?;

  Ok(())
}

async fn http_server() -> Result<(), Box<dyn std::error::Error>> {
  let addr = tokio::net::TcpListener::bind("0.0.0.0:8099").await.unwrap();
  let router = Router::new()
    // .route("/v1/auth/docs/", get(serve_swagger_ui))
    .route("/v1/auth/swagger.json", get(serve_openapi_spec));

  println!("HTTP server listening on {}", addr.local_addr().unwrap());

  axum::serve(addr, router).await.unwrap();

  Ok(())
}

// async fn serve_swagger_ui() -> Response {
  // let path = req.uri().path().trim_start_matches("/docs/");
  // let file_path = format!("./docs/{}", path);
  // let contents = match tokio::fs::read(file_path).await {
  //   Ok(contents) => contents,
  //   Err(_) => return Response::builder()
  //     .status(StatusCode::NOT_FOUND)
  //     .body(Body::from("File not found"))
  //     .unwrap(),
  // };

  
//   Ok()
// }

async fn serve_openapi_spec() -> Result<Response, impl IntoResponse> {
  let file_path = PathBuf::from("./docs/auth.swagger.json");
  
  match tokio::fs::read_to_string(file_path).await {
    Ok(contents) => {
      // Build the response with the JSON content
      Response::builder()
        .header(header::CONTENT_TYPE, "application/json")
        .body(contents.into())
        .map_err(|err| {
          (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to build response: {}", err),
          )
        })
    }
    Err(err) => {
      // Handle file read errors, e.g., file not found
      Err((
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        format!("Failed to read OpenAPI spec: {}", err),
      ))
    }
  }
}