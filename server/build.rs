use std::io::Result;
use std::process::Command;

fn main() -> Result<()> {
    // Check if protoc is available (try multiple locations)
    let protoc_paths = [
        "protoc",
        "/usr/local/bin/protoc",
        "/usr/bin/protoc",
        &format!("{}/bin/protoc", std::env::var("HOME").unwrap_or_default()),
    ];

    let protoc_path = protoc_paths
        .iter()
        .find(|path| Command::new(path).arg("--version").output().is_ok());

    if let Some(protoc) = protoc_path {
        // Generate Rust code from Protobuf definitions
        prost_build::Config::new()
            .prost_path((*protoc).to_string())
            .compile_protos(&["../proto/messages.proto"], &["../proto/"])?;
        println!("cargo:rerun-if-changed=../proto/messages.proto");
    } else {
        // protoc not available - skip generation for now
        println!("cargo:warning=protoc not found, skipping Protobuf generation");
        println!("cargo:warning=Install protobuf-compiler to enable Protobuf code generation");
    }

    Ok(())
}
