use crate::income::protos;

pub enum CalculateCommercialIncomeArgs {
    Monthly(protos::CommercialIncomeMonthlyArgs),
    Yearly(protos::CommercialIncomeYearlyArgs),
}

pub fn calculate_commercial_income(
    args: CalculateCommercialIncomeArgs,
) -> Result<protos::CalcIncomeResponse, String> {
    let args = match args {
        CalculateCommercialIncomeArgs::Monthly(args) => (
            args.total_sqft,
            args.income_potential_per_sqft_monthly,
            args.vacancy_rate_percentage,
        ),
        CalculateCommercialIncomeArgs::Yearly(args) => (
            args.total_sqft,
            args.income_potential_per_sqft_yearly,
            args.vacancy_rate_percentage,
        ),
    };

    match args {
        (sqft, potential_per_sqft, v_rate_percentage)
            if sqft > 0.0
                && potential_per_sqft >= 0.0
                && v_rate_percentage >= 0.0
                && v_rate_percentage <= 100.0 =>
        {
            let potential_income = sqft * potential_per_sqft;
            let vacant_sqft = sqft - (sqft * v_rate_percentage / 100.0);
            let unrealized_income = potential_income - vacant_sqft * potential_per_sqft;
            let realized_income = potential_income - unrealized_income;

            Ok(protos::CalcIncomeResponse {
                potential_income,
                realized_income,
                unrealized_income,
            })
        }

        (sqft, _, _) if sqft <= 0.0 => Err(String::from("total_sqft must be gt 0")),

        (_, income_potential_per_sqft_monthly, _) if income_potential_per_sqft_monthly < 0.0 => {
            Err(String::from(
                "income_potential_per_sqft must be gte 0",
            ))
        }

        (_, _, v_rate_percentage) if v_rate_percentage < 0.0 || v_rate_percentage > 100.0 => Err(
            String::from("vacancy_rate_percentage must be gte 0 and lte 100"),
        ),

        _ => Err(String::from("unknown error")),
    }
}
