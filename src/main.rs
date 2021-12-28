use aeonai_calcs::build_cost::grpc_service as build_cost_service;
use aeonai_calcs::income::grpc_service as income_service;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let port = match std::env::var("PORT") {
        Ok(port) => port,
        Err(_) => String::from("50051"),
    };

    let addr = format!("[::]:{}", port).parse()?;

    println!("Starting gRPC Server on {}", addr);
    Server::builder()
        .add_service(build_cost_service::new())
        .add_service(income_service::new())
        .serve(addr)
        .await?;

    Ok(())
}
