fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("api/v1/log.proto")?;
    Ok(())
}
