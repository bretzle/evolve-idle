use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Evolution {
    pub dna_unlocked: bool,
    pub membrane: Option<u32>,
    pub organelles: Option<u32>,
    pub nucleus: Option<u32>,
    pub eukaryotic_cell: Option<u32>,
    pub mitochondria: Option<u32>,

    pub sexual_reproduction: Option<bool>,
    pub phagocytosis: Option<bool>,
    pub chloroplasts: Option<bool>,
    pub chitin: Option<bool>,

    pub multicellular: Option<bool>,
    pub bilateral_symmetry: Option<bool>,
    pub poikilohydric: Option<bool>,
    pub spores: Option<bool>,

    pub bryophyte: Option<bool>,
    pub sentience: Option<bool>,

    pub progress: Option<u32>,
}

impl Evolution {
    pub fn new() -> Self {
        Self {
            dna_unlocked: false,
            membrane: None,
            organelles: None,
            nucleus: None,
            eukaryotic_cell: None,
            mitochondria: None,
            sexual_reproduction: None,
            phagocytosis: None,
            chloroplasts: None,
            chitin: None,
            multicellular: None,
            bilateral_symmetry: None,
            poikilohydric: None,
            spores: None,
            bryophyte: None,
            sentience: None,
            progress: None,
        }
    }
}
