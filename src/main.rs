use std::net::{IpAddr, Ipv4Addr, SocketAddr};

/* Import compiled proto */
use proto::auth_service_server::{ AuthService, AuthServiceServer };
use tonic::transport::Server;

mod proto {
  tonic::include_proto!("auth");

  pub(crate) const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("auth_descriptor");
}

mod logic {
  use diesel::{SelectableHelper, prelude::*};

}

#[derive(Debug, Default)]
struct AuthServiceImpl {}

#[tonic::async_trait]
impl AuthService for AuthServiceImpl {
  
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("Server running on port 50001");

  let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 50001); 
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
