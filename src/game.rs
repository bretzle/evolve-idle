use std::{
    collections::HashSet,
    fmt,
    ops::{AddAssign, SubAssign},
};

use crate::util::{Bounded, MutMap};

#[derive(Debug, Clone, Copy)]
pub struct Resource {
    pub amt: Bounded,
    pub display: bool,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.amt.get(), self.amt.max)
    }
}

impl<T: Into<f64> + Copy> AddAssign<T> for Resource {
    fn add_assign(&mut self, rhs: T) {
        self.amt.modify(|val| *val += rhs.into());
    }
}

impl<T: Into<f64> + Copy> SubAssign<T> for Resource {
    fn sub_assign(&mut self, rhs: T) {
        self.amt.modify(|val| *val -= rhs.into());
    }
}

macro_rules! map {
    ($( $key:expr => $val:expr ),* $(,)?) => {
        {
            let mut map = MutMap::new();
            $( map.insert($key, $val); )*
            map
        }
    };
}

#[derive(Clone, Copy)]
pub struct Cost {
    pub resource: &'static str,
    pub amount: f64,
}

pub enum GameStage {
    Evolution,
    Civilization,
}

pub struct Race {
    pub species: &'static str,
}

pub struct GameData {
    pub seed: u64,
    pub stage: GameStage,
    pub resource: MutMap<&'static str, Resource>,
    pub evolution: MutMap<&'static str, u32>,
    pub unlocks: HashSet<&'static str>,
    pub race: Race,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            seed: 1,
            stage: GameStage::Evolution,
            resource: map! {
                "RNA" => Resource { amt: Bounded::new(1000, 1000), display: false },
                "DNA" => Resource { amt: Bounded::new(10000, 10000), display: false },
            },
            evolution: map! {
                "membrane" => 0,
                "organelles" => 0,
                "nucleus" => 0,
                "eukaryotic_cell" => 0,
                "mitochondria" => 0,
                "sexual_reproduction" => 0,
            },
            unlocks: HashSet::new(),
            race: Race {
                species: "protoplasm",
            },
        }
    }

    pub fn afford(&self, costs: &[Cost]) -> bool {
        for Cost { resource, amount } in costs {
            if self.resource[resource].amt < *amount {
                return false;
            }
        }

        true
    }

    pub fn pay(&mut self, costs: &[Cost]) {
        for Cost { resource, amount } in costs {
            self.resource[resource] -= *amount;
        }
    }

    pub fn is_unlocked(&self, key: &'static str) -> bool {
        self.unlocks.contains(key)
    }

    pub fn unlock(&mut self, key: &'static str) {
        self.unlocks.insert(key);
    }

    pub fn lock(&mut self, key: &'static str) {
        self.unlocks.remove(key);
    }

    pub(crate) fn enter_sentience(&mut self) {
        // Should these just be hidden instead?
        self.stage = GameStage::Civilization;

        self.resource.remove("RNA");
        self.resource.remove("DNA");

        // setup race traits
    }
}
