use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let proto_file: &str = "./proto/auth.proto";
  let output_file = PathBuf::from(env::var("OUT_DIR").unwrap());

  tonic_build::configure()
    .file_descriptor_set_path(output_file.join("auth_descriptor.bin"))
    .compile(&[proto_file], &["proto"])?;
  // tonic_build::compile_protos(proto_file)?;

  tonic_build::configure()
    .protoc_arg("-I/usr/local/include/")
    .build_client(true)
    .build_server(true)
    .out_dir("./src")
    .compile(&[proto_file], &["proto"])?;

  Ok(())
}