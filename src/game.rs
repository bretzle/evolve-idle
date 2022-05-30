use crate::{
    structure::Structure,
    util::{Bounded, MutMap},
};
use std::{
    collections::HashSet,
    fmt,
    ops::{AddAssign, SubAssign},
};

#[derive(Debug, Clone, Copy)]
pub struct Resource {
    pub amount: f32,
    pub max: f32,
    pub delta: f32,
    pub display: bool,
}

impl Resource {
    pub fn is_full(&self) -> bool {
        self.amount >= self.max
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.amount, self.max)
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

macro_rules! set {
    ($( $val:expr ),* $(,)?) => {
        {
            let mut set = HashSet::new();
            $( set.insert($val); )*
            set
        }
    };
}

#[derive(Clone, Copy)]
pub struct Cost {
    pub resource: &'static str,
    pub amount: f32,
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
    pub evolution: MutMap<&'static str, u16>,
    pub unlocks: HashSet<&'static str>,
    pub race: Race,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            seed: 1,
            stage: GameStage::Evolution,
            resource: map! {
                "RNA" => Resource { amount: 0.0, max: 100.0, display: true, delta: 0.0 },
                "DNA" => Resource { amount: 0.0, max: 100.0, display: false, delta: 0.0 },
            },
            evolution: map! {
                "membrane" => 0,
                "organelles" => 0,
                "nucleus" => 0,
                "eukaryotic_cell" => 0,
                "mitochondria" => 0,
                "sexual_reproduction" => 0,
                "poikilohydric" => 0,
            },
            unlocks: set![crate::evolution::Rna::ID,],
            race: Race {
                species: "protoplasm",
            },
        }
    }

    pub fn afford(&self, costs: &[Cost]) -> bool {
        for Cost { resource, amount } in costs {
            if self.resource[resource].amount < *amount {
                return false;
            }
        }

        true
    }

    pub fn pay(&mut self, costs: &[Cost]) {
        for Cost { resource, amount } in costs {
            self.resource[resource].amount -= *amount;
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

        self.resource.clear();
        self.evolution.clear();
        self.unlocks.clear();

        // setup race traits
    }

    pub(crate) fn mod_res<T: Into<f32>>(
        &mut self,
        res: &'static str,
        val: T,
        notrack: bool,
        buffer: bool,
    ) -> bool {
        let val = val.into();
        let mut count = self.resource[res].amount + val;
        let mut success = true;

        if count > self.resource[res].max && self.resource[res].max != -1.0 {
            count = self.resource[res].max;
        } else if count < 0.0 {
            if !buffer || (buffer && (-count > buffer as u32 as f32)) {
                success = false;
            }
            count = 0.0;
        }

        if !count.is_nan() {
            self.resource[res].amount = count;
            if !notrack {
                self.resource[res].delta += val;
                // TODO: mana
            }
        }

        success
    }
}
