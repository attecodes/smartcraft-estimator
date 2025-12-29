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
