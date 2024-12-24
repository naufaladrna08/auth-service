use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let proto_file: &str = "./proto/auth.proto";
  let output_file = PathBuf::from(env::var("OUT_DIR").unwrap());

  // Generate gRPC client and server code
  tonic_build::configure()
    .file_descriptor_set_path(output_file.join("auth_descriptor.bin"))
    .compile(&[proto_file], &["proto"])?;

  // Generate OpenAPI v2 specification (swagger)
  let openapiv2_output = "./docs";
  std::fs::create_dir_all(openapiv2_output)?;

  let protoc_args = vec![
    format!("--plugin=protoc-gen-openapiv2={}", "/usr/local/bin/protoc-gen-openapiv2"),
    format!("-I{}", "proto"),
    format!("--openapiv2_out={}", openapiv2_output),
    proto_file.to_string(),
  ];

  // Execute protoc command
  let status = std::process::Command::new("protoc")
    .args(protoc_args)
    .status()?;

  if !status.success() {
    return Err(format!("protoc failed with status: {:?}", status).into());
  }
  
  Ok(())
}
