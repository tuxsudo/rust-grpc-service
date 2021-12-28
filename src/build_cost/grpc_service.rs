use crate::build_cost::calc;
use crate::build_cost::protos::{
    self,
    build_cost_calculator_server::{BuildCostCalculator, BuildCostCalculatorServer},
};
use tonic::{Code, Request, Response, Status};

#[derive(Debug, Default)]
pub struct BuildCostCalculatorService {}

#[tonic::async_trait]
impl BuildCostCalculator for BuildCostCalculatorService {
    async fn calc_building_sqft(
        &self,
        request: Request<protos::CalcBuildingSqftRequest>,
    ) -> Result<Response<protos::CalcBuildingSqftResponse>, Status> {
        match calc::calculate_building_sqft(&request.into_inner()) {
            Ok(results) => Ok(Response::new(results)),
            Err(msg) => Err(Status::new(Code::InvalidArgument, msg)),
        }
    }

    async fn calc_land_prep_costs(
        &self,
        request: Request<protos::CalcLandPrepCostsRequest>,
    ) -> Result<Response<protos::CalcLandPrepCostsResponse>, Status> {
        match calc::calculate_land_prep_costs(&request.into_inner()) {
            Ok(results) => Ok(Response::new(results)),
            Err(msg) => Err(Status::new(Code::InvalidArgument, msg)),
        }
    }

    async fn calc_structure_build_costs(
        &self,
        request: Request<protos::CalcStructureBuildCostsRequest>,
    ) -> Result<Response<protos::CalcStructureBuildCostsResponse>, Status> {
        match calc::calculate_structure_build_costs(&request.into_inner()) {
            Ok(results) => Ok(Response::new(results)),
            Err(msg) => Err(Status::new(Code::InvalidArgument, msg)),
        }
    }
}

pub fn new() -> BuildCostCalculatorServer<BuildCostCalculatorService> {
    BuildCostCalculatorServer::new(BuildCostCalculatorService::default())
}
