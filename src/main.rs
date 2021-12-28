use aeonai_calcs::build_cost::grpc_service;
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
        .add_service(grpc_service::new())
        .serve(addr)
        .await?;

    Ok(())
}
