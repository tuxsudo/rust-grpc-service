use crate::income::calc;
use crate::income::protos::{
    self,
    calc_income_request::Income,
    income_calculator_server::{IncomeCalculator, IncomeCalculatorServer},
};
use tonic::{Code, Request, Response, Status};

#[derive(Debug, Default)]
pub struct IncomeCalculatorService {}

#[tonic::async_trait]
impl IncomeCalculator for IncomeCalculatorService {
    async fn calc_income(
        &self,
        request: Request<protos::CalcIncomeRequest>,
    ) -> Result<Response<protos::CalcIncomeResponse>, Status> {
        let response = match request.into_inner().income {
            Some(Income::Monthly(args)) => calc::calculate_commercial_income(
                calc::CalculateCommercialIncomeArgs::Monthly(args),
            ),

            Some(Income::Yearly(args)) => {
                calc::calculate_commercial_income(calc::CalculateCommercialIncomeArgs::Yearly(args))
            }

            _ => Err(String::from("Invalid request")),
        };

        match response {
            Ok(results) => Ok(Response::new(results)),
            Err(msg) => Err(Status::new(Code::InvalidArgument, msg)),
        }
    }
}

pub fn new() -> IncomeCalculatorServer<IncomeCalculatorService> {
    IncomeCalculatorServer::new(IncomeCalculatorService::default())
}
