//! Generates the Rust code for the `.proto` files. Requires `protoc` on the build machine.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/users/v1/users.proto")?;
    Ok(())
}
