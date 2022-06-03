use enum_iterator::Sequence;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    ops::{Index, IndexMut},
};

#[derive(Clone, Copy, Sequence)]
pub enum ResourceType {
    RNA,
    DNA,
}

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            ResourceType::RNA => "RNA",
            ResourceType::DNA => "DNA",
        };
        write!(f, "{name}")
    }
}

#[derive(Serialize, Deserialize)]
pub struct Resource {
    pub amount: f32,
    pub max: f32,
    pub delta: f32,
    pub diff: f32,
    pub rate: f32,
    pub display: bool,
}

impl Resource {
    pub fn new(amount: f32, max: f32, rate: f32, display: bool) -> Self {
        Self {
            amount,
            max,
            delta: 0.0,
            diff: 0.0,
            rate,
            display,
        }
    }

    pub fn is_full(&self) -> bool {
        self.amount >= self.max
    }
}

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct Resources {
    pub rna: Resource,
    pub dna: Resource,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            rna: Resource::new(0.0, 100.0, 1.0, true),
            dna: Resource::new(0.0, 100.0, 1.0, false),
        }
    }
}

impl Index<ResourceType> for Resources {
    type Output = Resource;

    fn index(&self, index: ResourceType) -> &Self::Output {
        match index {
            ResourceType::RNA => &self.rna,
            ResourceType::DNA => &self.dna,
        }
    }
}

impl IndexMut<ResourceType> for Resources {
    fn index_mut(&mut self, index: ResourceType) -> &mut Self::Output {
        match index {
            ResourceType::RNA => &mut self.rna,
            ResourceType::DNA => &mut self.dna,
        }
    }
}
