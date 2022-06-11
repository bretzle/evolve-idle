use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Species {
    Protoplasm,
	// Fungi
    Sporgar,
    Shroomi,
    Molding,
	// Plants
    Entish,
    Cacti,
    Pinguicula,
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
