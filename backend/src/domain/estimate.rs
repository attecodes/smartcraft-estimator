pub struct Measurement {
    pub linear_ft: Option<f64>,
    pub square_ft: Option<f64>,
}
pub enum PricingUnit {
    LinearFoot,
    SquareFoot,
    Unit,
}

pub struct Material {
    pub name: String,
    pub cost_per_unit: f64,
    pub pricing_unit: PricingUnit,
    pub waste_factor: f64, // e.g. 0.10 = 10%
    pub units_per_measure: Option<f64>,

}
pub struct MaterialLineItem {
    pub material: Material,
    pub measurement: Measurement,
}

impl Material {
    pub fn base_quantity(&self, measurement: &Measurement) -> f64 {
        match self.pricing_unit {
            PricingUnit::LinearFoot => {
                measurement.linear_ft.unwrap_or(0.0)
            }
            PricingUnit::SquareFoot => {
                measurement.square_ft.unwrap_or(0.0)
            }
            PricingUnit::Unit => {
                let units_per_measure = self.units_per_measure.unwrap_or(1.0);

                let measure = measurement.square_ft
                    .or(measurement.linear_ft)
                    .unwrap_or(0.0);

                measure * units_per_measure
            }
        }
    }
}

impl MaterialLineItem {
    pub fn base_quantity(&self) -> f64 {
        self.material.base_quantity(&self.measurement)
    }

    pub fn effective_quantity(&self) -> f64 {
        self.base_quantity() * (1.0 + self.material.waste_factor)
    }

    pub fn cost(&self) -> f64 {
        self.effective_quantity() * self.material.cost_per_unit
    }
}
pub struct Labor {
    pub hours: f64,
    pub rate_per_hour: f64,
}

impl Labor {
    pub fn cost(&self) -> f64 {
        self.hours * self.rate_per_hour
    }
}
pub struct Estimate {
    pub materials: Vec<MaterialLineItem>,
    pub labor: Labor,
    pub margin: f64, // 0.20 = 20%
}
impl Estimate {
    pub fn material_cost(&self) -> f64 {
        self.materials.iter().map(|m| m.cost()).sum()
    }

    pub fn labor_cost(&self) -> f64 {
        self.labor.cost()
    }

    pub fn subtotal(&self) -> f64 {
        self.material_cost() + self.labor_cost()
    }

    pub fn total(&self) -> f64 {
        let subtotal = self.subtotal();
        subtotal + (subtotal * self.margin)
    }
}
