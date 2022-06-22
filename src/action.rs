use crate::race::Species;
use crate::resource::ResourceType::*;
use crate::{loc, ACTIONS};
use crate::{resource::Cost, Game};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::ops::Index;

type Hook<T = ()> = fn(&Game) -> T;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Category {
    Evolution,
}

#[derive(Clone)]
pub struct Action {
    pub id: &'static str,
    title: &'static str,
    desc: &'static str,
    effect: Option<Hook<Cow<'static, str>>>,
    cost: Option<Hook<Vec<Cost>>>,
    action: fn(&Self, &mut Game),
    count: Option<Hook<Option<u32>>>,
}

impl std::fmt::Debug for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Action").field("id", &self.id).finish()
    }
}

impl Action {
    pub fn title(&self) -> Cow<str> {
        loc!(self.title)
    }

    pub fn description(&self) -> Cow<str> {
        loc!(self.desc)
    }

    pub fn effect(&self, game: &Game) -> Option<Cow<str>> {
        self.effect.map(|hook| hook(game))
    }

    pub fn cost(&self, game: &Game) -> Vec<Cost> {
        self.cost.map(|hook| hook(game)).unwrap_or_default()
    }

    pub fn execute(&self, game: &mut Game) {
        (self.action)(self, game)
    }

    pub fn count(&self, game: &Game) -> Option<u32> {
        self.count.map(|hook| hook(game)).flatten()
    }

    fn pay(&self, game: &mut Game) -> bool {
        let costs = self.cost(game);
        if game.check_costs(&costs) {
            for cost in costs {
                let Cost { resource, amount } = cost;
                game.resources[resource].amount -= amount;
                // TODO: update stats
            }

            return true;
        }

        false
    }
}

#[derive(Debug)]
pub struct ActionHolder {
    inner: HashMap<Category, Vec<Action>>,
    unlocks: HashSet<&'static str>,
}

impl ActionHolder {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            unlocks: HashSet::new(),
        }
    }

    pub fn add(&mut self, category: Category, action: Action) {
        self.unlocks.insert(action.id);
        self.inner.entry(category).or_default().push(action);
    }

    pub fn remove(&mut self, category: Category, id: &'static str) {
        if let Some(actions) = self.inner.get_mut(&category) {
            actions.retain(|action| action.id != id)
        }
    }

    pub fn clear(&mut self) {
        self.inner.clear();
        self.unlocks.clear();

        self.add(Category::Evolution, ACTION_RNA);
    }

    pub fn unlocked(&mut self, action: Action) -> bool {
        self.unlocks.contains(action.id)
    }
}

impl Index<Category> for ActionHolder {
    type Output = Vec<Action>;

    fn index(&self, index: Category) -> &Self::Output {
        &self.inner[&index]
    }
}

macro_rules! cost {
    ( $game:expr, $($resource:ident => $cell:tt, $base:expr, $mult:expr),* ) => {
        vec![$(
            Cost { resource: $resource, amount: ($game.evolution.$cell.unwrap() * $mult + $base) as _ }
        ),*]
    };

    ($( $resource:ident => $amt:expr ),* ) => {
        vec![$( Cost { resource:$resource, amount:$amt as _ } ),*]
    }
}

macro_rules! inc {
	($($t:tt)+) => {
		if let Some(count) = $($t)+.as_mut() {
			*count += 1;
		}
	};
}

pub const ACTION_RNA: Action = Action {
    id: "evolution-rna",
    title: "resource_RNA_name",
    desc: "evo_rna",
    effect: None,
    cost: None,
    action: |_, game| {
        if !game.resources.rna.is_full() {
            game.mod_res(RNA, 1.0, true, false);
        }
    },
    count: None,
};

pub const ACTION_DNA: Action = Action {
    id: "evolution-dna",
    title: "evo_dna_title",
    desc: "evo_dna_desc",
    effect: Some(|_| loc!("evo_dna_effect")),
    cost: Some(|_| cost!(RNA => 2)),
    action: |_, game| {
        if game.resources[RNA].amount >= 2.0 && !game.resources[DNA].is_full() {
            game.mod_res(RNA, -2.0, true, false);
            game.mod_res(DNA, 1.0, true, false);
        }
    },
    count: None,
};

pub const ACTION_MEMBRANE: Action = Action {
    id: "evolution-membrane",
    title: "evo_membrane_title",
    desc: "evo_membrane_desc",
    effect: Some(|game| {
        let effect = game.evolution.mitochondria.map(|x| x * 5 + 5).unwrap_or(5);
        loc!("evo_membrane_effect", effect)
    }),
    cost: Some(|game| cost!(game, RNA => membrane, 2, 2)),
    action: |s, game| {
        if s.pay(game) {
            game.resources.rna.max += match game.evolution.mitochondria {
                Some(count) => count * 5 + 5,
                None => 5,
            } as f32;
            inc!(game.evolution.membrane);
        }
    },
    count: Some(|game| game.evolution.membrane),
};

pub const ACTION_ORGANELLES: Action = Action {
    id: "evolution-organelles",
    title: "evo_organelles_title",
    desc: "evo_organelles_desc",
    effect: Some(|game| {
        let mut rna = 1;
        if game.evolution.sexual_reproduction == Some(true) {
            rna += 1;
        }
        loc!("evo_organelles_effect", rna)
    }),
    cost: Some(|game| {
        cost! {
            game,
            RNA => organelles, 12, 8,
            DNA => organelles, 4, 4
        }
    }),
    action: |s, game| {
        if s.pay(game) {
            inc!(game.evolution.organelles);
        }
    },
    count: Some(|game| game.evolution.organelles),
};

pub const ACTION_NUCLEUS: Action = Action {
    id: "evolution-nucleus",
    title: "evo_nucleus_title",
    desc: "evo_nucleus_desc",
    effect: Some(|_| {
        // TODO: bilateral_symmetry, poikilohydric, spores increase this
        let dna = if false { 2 } else { 1 };
        loc!("evo_nucleus_effect", dna)
    }),
    cost: Some(|game| {
        let multi = game.evolution.multicellular == Some(true);
        cost! {
            game,
            RNA => nucleus, 38, if multi { 16 } else { 32 },
            DNA => nucleus, 18, if multi { 12 } else { 16 }
        }
    }),
    action: |s, game| {
        if s.pay(game) {
            inc!(game.evolution.nucleus);
        }
    },
    count: Some(|game| game.evolution.nucleus),
};

pub const ACTION_EUKARYOTIC_CELL: Action = Action {
    id: "evolution-eukaryotic_cell",
    title: "evo_eukaryotic_title",
    desc: "evo_eukaryotic_desc",
    effect: Some(|game| {
        let effect = match game.evolution.mitochondria {
            Some(count) => count * 10 + 10,
            None => 10,
        };
        loc!("evo_eukaryotic_effect", effect)
    }),
    cost: Some(|game| {
        cost! {
            game,
            RNA => eukaryotic_cell, 20, 20,
            DNA => eukaryotic_cell, 40, 12
        }
    }),
    action: |s, game| {
        if s.pay(game) {
            inc!(game.evolution.eukaryotic_cell);
            game.resources.dna.max += match game.evolution.mitochondria {
                Some(count) => count * 10 + 10,
                None => 10,
            } as f32;
        }
    },
    count: Some(|game| game.evolution.eukaryotic_cell),
};

pub const ACTION_MITOCHONDRIA: Action = Action {
    id: "evolution-mitochondria",
    title: "evo_mitochondria_title",
    desc: "evo_mitochondria_desc",
    effect: Some(|_| loc!("evo_mitochondria_effect")),
    cost: Some(|game| {
        cost! {
            game,
            RNA => mitochondria, 75, 50,
            DNA => mitochondria, 65, 35
        }
    }),
    action: |s, game| {
        if s.pay(game) {
            inc!(game.evolution.mitochondria);
        }
    },
    count: Some(|game| game.evolution.mitochondria),
};

pub const ACTION_SEXUAL_REPRODUCTION: Action = Action {
    id: "evolution-sexual_reproduction",
    title: "evo_sexual_reproduction_title",
    desc: "evo_sexual_reproduction_desc",
    effect: Some(|_| loc!("evo_sexual_reproduction_effect")),
    cost: Some(|_| cost!(DNA => 150)),
    action: |s, game| {
        if s.pay(game) {
            let mut holder = ACTIONS.lock().unwrap();

            game.evolution.sexual_reproduction = Some(true);
            game.evolution.phagocytosis = Some(false);
            game.evolution.chloroplasts = Some(false);
            game.evolution.chitin = Some(false);

            holder.remove(Category::Evolution, ACTION_SEXUAL_REPRODUCTION.id);
            holder.add(Category::Evolution, ACTION_PHAGOCYTOSIS);
            holder.add(Category::Evolution, ACTION_CHLOROPLASTS);
            holder.add(Category::Evolution, ACTION_CHITIN);

            game.evolution.progress = Some(20);
        }
    },
    count: None,
};

/////////////////////////////////////////////////////////////

pub const ACTION_PHAGOCYTOSIS: Action = Action {
    id: "evolution-phagocytosis",
    title: "evo_phagocytosis_title",
    desc: "evo_phagocytosis_desc",
    effect: Some(|_| loc!("evo_phagocytosis_effect")),
    cost: Some(|_| cost!(DNA => 175)),
    action: |s, game| {
        if s.pay(game) {
            let mut holder = ACTIONS.lock().unwrap();

            holder.remove(Category::Evolution, ACTION_PHAGOCYTOSIS.id);
            holder.remove(Category::Evolution, ACTION_CHLOROPLASTS.id);
            holder.remove(Category::Evolution, ACTION_CHITIN.id);
            holder.add(Category::Evolution, ACTION_MULTICELLULAR);

            game.evolution.phagocytosis = Some(true);
            game.evolution.chloroplasts = None;
            game.evolution.chitin = None;
            game.evolution.multicellular = Some(false);
            game.evolution.progress = Some(40);
        }
    },
    count: None,
};

pub const ACTION_CHLOROPLASTS: Action = Action {
    id: "evolution-chloroplasts",
    title: "evo_chloroplasts_title",
    desc: "evo_chloroplasts_desc",
    effect: Some(|_| loc!("evo_chloroplasts_effect")),
    cost: Some(|_| cost!(DNA => 175)),
    action: |s, game| {
        if s.pay(game) {
            let mut holder = ACTIONS.lock().unwrap();

            holder.remove(Category::Evolution, ACTION_PHAGOCYTOSIS.id);
            holder.remove(Category::Evolution, ACTION_CHLOROPLASTS.id);
            holder.remove(Category::Evolution, ACTION_CHITIN.id);
            holder.add(Category::Evolution, ACTION_MULTICELLULAR);

            game.evolution.chloroplasts = Some(true);
            game.evolution.phagocytosis = None;
            game.evolution.chitin = None;
            game.evolution.multicellular = Some(false);
            game.evolution.progress = Some(40);
        }
    },
    count: None,
};

pub const ACTION_CHITIN: Action = Action {
    id: "evolution-chitin",
    title: "evo_chitin_title",
    desc: "evo_chitin_desc",
    effect: Some(|_| loc!("evo_chitin_effect")),
    cost: Some(|_| cost!(DNA => 175)),
    action: |s, game| {
        if s.pay(game) {
            let mut holder = ACTIONS.lock().unwrap();

            holder.remove(Category::Evolution, ACTION_PHAGOCYTOSIS.id);
            holder.remove(Category::Evolution, ACTION_CHLOROPLASTS.id);
            holder.remove(Category::Evolution, ACTION_CHITIN.id);
            holder.add(Category::Evolution, ACTION_MULTICELLULAR);

            game.evolution.chitin = Some(true);
            game.evolution.phagocytosis = None;
            game.evolution.chloroplasts = None;
            game.evolution.multicellular = Some(false);
            game.evolution.progress = Some(40);
        }
    },
    count: None,
};

pub const ACTION_MULTICELLULAR: Action = Action {
    id: "evolution-multicellular",
    title: "evo_multicellular_title",
    desc: "evo_multicellular_desc",
    effect: Some(|_| loc!("evo_multicellular_effect")),
    cost: Some(|_| cost!(DNA => 200)),
    action: |s, game| {
        if s.pay(game) {
            let mut holder = ACTIONS.lock().unwrap();

            holder.remove(Category::Evolution, ACTION_MULTICELLULAR.id);

            game.evolution.multicellular = Some(true);
            game.evolution.progress = Some(60);

            if game.evolution.phagocytosis.is_some() {
                holder.add(Category::Evolution, ACTION_BILATERAL_SYMMETRY);
                game.evolution.bilateral_symmetry = Some(false);
            } else if game.evolution.chloroplasts.is_some() {
                holder.add(Category::Evolution, ACTION_POKILOHYDRIC);
                game.evolution.poikilohydric = Some(false);
            } else if game.evolution.chitin.is_some() {
                holder.add(Category::Evolution, ACTION_SPORES);
                game.evolution.spores = Some(false);
            }
        }
    },
    count: None,
};

/////////////////////////////////////////////////////////////

pub const ACTION_BILATERAL_SYMMETRY: Action = Action {
    id: "evolution-bilateral_symmetry",
    title: "evo_bilateral_symmetry_title",
    desc: "evo_bilateral_symmetry_desc",
    effect: Some(|_| loc!("evo_nucleus_boost")),
    cost: Some(|_| cost!(DNA => 230)),
    action: |_, _| panic!("Animal kingdom is not implemented yet"),
    count: None,
};

pub const ACTION_POKILOHYDRIC: Action = Action {
    id: "evolution-poikilohydric",
    title: "evo_poikilohydric_title",
    desc: "evo_poikilohydric_desc",
    effect: Some(|_| loc!("evo_nucleus_boost")),
    cost: Some(|_| cost!(DNA => 230)),
    action: |s, game| {
        if s.pay(game) {
            let mut holder = ACTIONS.lock().unwrap();

            holder.remove(Category::Evolution, ACTION_POKILOHYDRIC.id);
            holder.add(Category::Evolution, ACTION_BRYOPHYTE);

            game.evolution.poikilohydric = Some(true);
            game.evolution.bryophyte = Some(false);
            game.evolution.progress = Some(80);
        }
    },
    count: None,
};

pub const ACTION_SPORES: Action = Action {
    id: "evolution-spores",
    title: "evo_spores_title",
    desc: "evo_spores_desc",
    effect: Some(|_| loc!("evo_nucleus_boost")),
    cost: Some(|_| cost!(DNA => 230)),
    action: |s, game| {
        if s.pay(game) {
            let mut holder = ACTIONS.lock().unwrap();

            holder.remove(Category::Evolution, ACTION_SPORES.id);
            holder.add(Category::Evolution, ACTION_BRYOPHYTE);

            game.evolution.spores = Some(true);
            game.evolution.bryophyte = Some(false);
            game.evolution.progress = Some(80);
        }
    },
    count: None,
};

/////////////////////////////////////////////////////////////

pub const ACTION_BRYOPHYTE: Action = Action {
    id: "evolution-bryophyte",
    title: "evo_bryophyte_title",
    desc: "evo_bryophyte_desc",
    effect: Some(|_| loc!("evo_bryophyte_effect")),
    cost: Some(|_| cost!(DNA => 260)),
    action: |s, game| {
        if s.pay(game) {
            let mut holder = ACTIONS.lock().unwrap();

            holder.remove(Category::Evolution, ACTION_BRYOPHYTE.id);
            holder.add(Category::Evolution, ACTION_SENTIENCE);

            game.evolution.bryophyte = Some(true);
            game.evolution.progress = Some(100);
            game.evolution.sentience = Some(false);
        }
    },
    count: None,
};

pub const ACTION_SENTIENCE: Action = Action {
    id: "evolution-sentience",
    title: "evo_sentience_title",
    desc: "evo_sentience_desc",
    effect: Some(|_| loc!("evo_sentience_effect")),
    cost: Some(|_| {
        cost! {
            RNA => 300,
            DNA => 300
        }
    }),
    action: |s, game| {
        if s.pay(game) {
            let mut holder = ACTIONS.lock().unwrap();

            holder.remove(Category::Evolution, ACTION_SENTIENCE.id);

            game.evolution.sentience = Some(true);

            let mut races = vec![];
            if game.evolution.chitin.is_some() {
                races.extend([Species::Sporgar, Species::Shroomi, Species::Molding]);
            } else if game.evolution.chloroplasts.is_some() {
                races.extend([Species::Entish, Species::Cacti, Species::Pinguicula]);
            } else {
                unreachable!()
            }

            game.race.species = races[game.rng.usize(0..races.len())];

            game.become_sentient()
        }
    },
    count: None,
};
