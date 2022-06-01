#[derive(PartialEq)]
pub enum Species {
    Protoplasm,
}

pub struct Race {
    pub species: Species,
}

impl Default for Race {
    fn default() -> Self {
        Self {
            species: Species::Protoplasm,
        }
    }
}
