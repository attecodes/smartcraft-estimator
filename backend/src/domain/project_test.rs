#[test]
fn project_total_matches_estimate_total() {
    let material = Material {
        name: "Baseboard".to_string(),
        cost_per_unit: 2.0,
        pricing_unit: PricingUnit::LinearFoot,
        waste_factor: 0.10,
        units_per_measure: None,
    };

    let item = MaterialLineItem {
        material,
        measurement: Measurement {
            linear_ft: Some(100.0),
            square_ft: None,
        },
    };

    let labor = Labor {
        hours: 8.0,
        rate_per_hour: 60.0,
    };

    let estimate = Estimate {
        materials: vec![item],
        labor,
        margin: 0.15,
    };

    let project = Project {
        id: Uuid::new_v4(),
        name: "Trim Install".to_string(),
        description: None,
        client_name: None,
        estimate,
    };

    assert_eq!(project.total(), estimate.total());
}
