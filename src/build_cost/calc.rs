use crate::build_cost::protos;

pub fn calculate_building_sqft(
    args: &protos::CalcBuildingSqftRequest,
) -> Result<protos::CalcBuildingSqftResponse, String> {
    match (args.building_footprint_sqft, args.stories as f32) {
        (b, s) if b > 0.0 && s >= 1.0 => Ok(protos::CalcBuildingSqftResponse { total_sqft: b * s }),

        (_, s) if s <= 1.0 => Err(String::from("stories must be gte 1")),

        (b, _) if b <= 0.0 => Err(String::from("footprint must be gt 0")),

        _ => Err(String::from("unknown error")),
    }
}

pub fn calculate_land_prep_costs(
    args: &protos::CalcLandPrepCostsRequest,
) -> Result<protos::CalcLandPrepCostsResponse, String> {
    match (args.grading_cost_per_sqft, args.total_sqft) {
        (g, t) if t >= 0.0 => Ok(protos::CalcLandPrepCostsResponse { total: g * t }),
        _ => Err(String::from("total_sqft must be gte 0")),
    }
}

pub fn calculate_structure_build_costs(
    args: &protos::CalcStructureBuildCostsRequest,
) -> Result<protos::CalcStructureBuildCostsResponse, String> {
    match (
        args.soft_cost_per_sqft,
        args.hard_cost_per_sqft,
        args.total_sqft,
    ) {
        (s, h, t) if t >= 0.0 => Ok(protos::CalcStructureBuildCostsResponse {
            total: s * t + h * t,
        }),
        _ => Err(String::from("total_sqft must be gte 0")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_building_sqft() {
        let results = calculate_building_sqft(&protos::CalcBuildingSqftRequest {
            building_footprint_sqft: 3.0,
            stories: 3,
        });

        assert_eq!(
            results,
            Ok(protos::CalcBuildingSqftResponse { total_sqft: 9.0 })
        );
    }
}
