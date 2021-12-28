pub mod calc;
pub mod grpc_service;

pub mod protos {
    tonic::include_proto!("build_cost");
}
