use uuid::Uuid;

pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub client_name: Option<String>,
    pub estimate: Estimate,
}
impl Project {
    pub fn subtotal(&self) -> f64 {
        self.estimate.subtotal()
    }

    pub fn total(&self) -> f64 {
        self.estimate.total()
    }
}
