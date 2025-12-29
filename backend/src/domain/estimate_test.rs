#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_material_cost_correctly() {
        let material = Material {
            name: "2x4 Lumber".to_string(),
            cost_per_unit: 3.0,
            pricing_unit: PricingUnit::LinearFoot,
            waste_factor: 0.1,
        };

        let item = MaterialLineItem {
            material,
            quantity: 100.0,
        };

        assert_eq!(item.cost(), 300.0);
    }

    #[test]
    fn calculates_total_estimate_with_margin() {
        let materials = vec![
            MaterialLineItem {
                material: Material {
                    name: "Plywood".to_string(),
                    cost_per_unit: 50.0,
                    pricing_unit: PricingUnit::Unit,
                    waste_factor: 0.0,
                },
                quantity: 4.0,
            }
        ];

        let labor = Labor {
            hours: 10.0,
            rate_per_hour: 60.0,
        };

        let estimate = Estimate {
            materials,
            labor,
            margin: 0.20,
        };

        assert_eq!(estimate.total(), 840.0);
    }
}
#[test]
fn applies_waste_factor_correctly() {
    let material = Material {
        name: "Deck Boards".to_string(),
        cost_per_unit: 5.0,
        pricing_unit: PricingUnit::LinearFoot,
        waste_factor: 0.15,
    };

    let item = MaterialLineItem {
        material,
        base_quantity: 200.0,
    };

    assert_eq!(item.effective_quantity(), 230.0);
    assert_eq!(item.cost(), 1150.0);
}
#[test]
fn calculates_linear_material_quantity() {
    let material = Material {
        name: "Baseboard".to_string(),
        cost_per_unit: 2.5,
        pricing_unit: PricingUnit::LinearFoot,
        waste_factor: 0.10,
        units_per_measure: None,
    };

    let item = MaterialLineItem {
        material,
        measurement: Measurement {
            linear_ft: Some(50.0),
            square_ft: None,
        },
    };

    assert_eq!(item.base_quantity(), 50.0);
    assert_eq!(item.effective_quantity(), 55.0);
}
#[test]
fn calculates_unit_based_material_quantity() {
    let material = Material {
        name: "Stud".to_string(),
        cost_per_unit: 4.0,
        pricing_unit: PricingUnit::Unit,
        waste_factor: 0.10,
        units_per_measure: Some(0.75),
    };

    let item = MaterialLineItem {
        material,
        measurement: Measurement {
            linear_ft: Some(100.0),
            square_ft: None,
        },
    };

    assert_eq!(item.base_quantity(), 75.0);
}