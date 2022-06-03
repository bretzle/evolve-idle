use serde::{Serialize, Deserialize};

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Species {
    Protoplasm,
}

#[derive(Serialize, Deserialize)]
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
