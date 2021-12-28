fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("./src/build_cost/build_cost.proto")?;
    tonic_build::compile_protos("./src/income/income.proto")?;
    Ok(())
}
