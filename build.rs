fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./src/build_cost/build_cost.proto")?;
    Ok(())
}
