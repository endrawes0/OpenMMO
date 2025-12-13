use std::io::Result;
use std::process::Command;

fn main() -> Result<()> {
    // Check if protoc is available
    if Command::new("protoc").arg("--version").output().is_ok() {
        // Generate Rust code from Protobuf definitions
        prost_build::compile_protos(&["../proto/messages.proto"], &["../proto/"])?;
        println!("cargo:rerun-if-changed=../proto/messages.proto");
    } else {
        // protoc not available - skip generation for now
        println!("cargo:warning=protoc not found, skipping Protobuf generation");
        println!("cargo:warning=Install protobuf-compiler to enable Protobuf code generation");
    }

    Ok(())
}
