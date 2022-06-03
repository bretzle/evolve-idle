use crate::{
    resource::ResourceType::*,
    structure::{Cost, Structure},
    Game,
};
use serde::{Deserialize, Serialize};

macro_rules! cost {
    ( $game:expr, $($resource:ident => $cell:tt, $base:expr, $mult:expr),* ) => {
        [$(
            Cost { resource: $resource, amount: ($game.evolution.$cell.unwrap() * $mult + $base) as _ }
        ),*]
    };

    ($( $resource:ident => $amt:expr ),* ) => {
        [$( Cost { resource:$resource, amount:$amt as _ } ),*]
    }
}

macro_rules! inc {
	($($t:tt)+) => {
		if let Some(count) = $($t)+.as_mut() {
			*count += 1;
		}
	};
}

#[derive(Serialize, Deserialize)]
pub struct Evolution {
    pub dna_unlocked: bool,
    pub membrane: Option<u32>,
    pub organelles: Option<u32>,
    pub nucleus: Option<u32>,
    pub eukaryotic_cell: Option<u32>,
    pub mitochondria: Option<u32>,
    pub sexual_reproduction: Option<bool>,
    pub multicellular: Option<bool>,
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
            multicellular: None,
        }
    }

    pub(crate) fn is_unlocked(&self, id: &str) -> bool {
        match id {
            "rna" => true,
            "dna" => self.dna_unlocked,
            "membrane" => self.membrane.is_some(),
            "organelles" => self.organelles.is_some(),
            "nucleus" => self.nucleus.is_some(),
            "eukaryotic_cell" => self.eukaryotic_cell.is_some(),
            "mitochondria" => self.mitochondria.is_some(),
            "sexual_reproduction" => self.sexual_reproduction == Some(true),
            "multicellular" => self.multicellular.is_some(),
            _ => unreachable!(),
        }
    }
}

fn pay<T: Structure>(game: &mut Game) -> bool
where
    [(); T::SIZE]:,
{
    let costs = T::cost(game);
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

//////////////////////////////////////////////

pub struct Rna;
impl Structure for Rna {
    const ID: &'static str = "rna";
    const SIZE: usize = 0;

    fn title() -> &'static str {
        "RNA"
    }

    fn cost(_: &Game) -> [Cost; Self::SIZE] {
        []
    }

    fn effect(_: &Game) -> String {
        "Creates 1 RNA".to_string()
    }

    fn description() -> &'static str {
        "Form new RNA"
    }

    fn action(game: &mut Game) {
        if !game.resources.rna.is_full() {
            game.mod_res(RNA, 1.0, true, false);
        }
    }

    fn tooltip(ui: &imgui::Ui, game: &Game) {
        ui.tooltip_text(Self::effect(game));
    }
}

//////////////////////////////////////////////

pub struct Dna;
impl Structure for Dna {
    const ID: &'static str = "dna";
    const SIZE: usize = 1;

    fn title() -> &'static str {
        "Form DNA"
    }

    fn cost(_: &Game) -> [Cost; Self::SIZE] {
        cost!(RNA => 2)
    }

    fn effect(_: &Game) -> String {
        "Turn 2 RNA into 1 DNA".to_string()
    }

    fn description() -> &'static str {
        "Creates a new strand of DNA"
    }

    fn action(game: &mut Game) {
        if game.resources[RNA].amount >= 2.0 && !game.resources[DNA].is_full() {
            game.mod_res(RNA, -2.0, true, false);
            game.mod_res(DNA, 1.0, true, false);
        }
    }
}

//////////////////////////////////////////////

pub struct Membrane;
impl Structure for Membrane {
    const ID: &'static str = "membrane";
    const SIZE: usize = 1;

    fn title() -> &'static str {
        "Membrane"
    }

    fn cost(game: &Game) -> [Cost; Self::SIZE] {
        cost!(game, RNA => membrane, 2, 2)
    }

    fn effect(game: &Game) -> String {
        let effect = match game.evolution.mitochondria {
            Some(count) => count * 5 + 5,
            None => 5,
        };
        format!("Increases RNA capacity by {effect}")
    }

    fn description() -> &'static str {
        "Evolve Membranes"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            game.resources.rna.max += match game.evolution.mitochondria {
                Some(count) => count * 5 + 5,
                None => 5,
            } as f32;
            inc!(game.evolution.membrane);
        }
    }
}

//////////////////////////////////////////////

pub struct Organelles;
impl Structure for Organelles {
    const ID: &'static str = "organelles";
    const SIZE: usize = 2;

    fn title() -> &'static str {
        "Organelles"
    }

    fn cost(game: &Game) -> [Cost; Self::SIZE] {
        cost! {
            game,
            RNA => organelles, 12, 8,
            DNA => organelles, 4, 4
        }
    }

    fn effect(game: &Game) -> String {
        let mut rna = 1;
        if game.evolution.sexual_reproduction == Some(true) {
            rna += 1;
        }
        format!("Automatically generate {rna} RNA")
    }

    fn description() -> &'static str {
        "Evolve Organelles"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            inc!(game.evolution.organelles);
        }
    }
}

//////////////////////////////////////////////

pub struct Nucleus;
impl Structure for Nucleus {
    const ID: &'static str = "nucleus";
    const SIZE: usize = 2;

    fn title() -> &'static str {
        "Nucleus"
    }

    fn cost(game: &Game) -> [Cost; Self::SIZE] {
        let multi = game.evolution.multicellular == Some(true);
        cost! {
            game,
            RNA => nucleus, 38, if multi { 16 } else { 32 },
            DNA => nucleus, 18, if multi { 12 } else { 16 }
        }
    }

    fn effect(_: &Game) -> String {
        // TODO: bilateral_symmetry, poikilohydric, spores increase this
        let dna = if false { 2 } else { 1 };
        format!("Automatically consume 2 RNA to create {dna} DNA")
    }

    fn description() -> &'static str {
        "Evolve Nucleus"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            inc!(game.evolution.nucleus)
        }
    }
}

//////////////////////////////////////////////

pub struct EukaryoticCell;
impl Structure for EukaryoticCell {
    const ID: &'static str = "eukaryotic_cell";
    const SIZE: usize = 2;

    fn title() -> &'static str {
        "Eukaryotic Cell"
    }

    fn cost(game: &Game) -> [Cost; Self::SIZE] {
        cost! {
            game,
            RNA => eukaryotic_cell, 20, 20,
            DNA => eukaryotic_cell, 40, 12
        }
    }

    fn effect(game: &Game) -> String {
        let effect = match game.evolution.mitochondria {
            Some(count) => count * 10 + 10,
            None => 10,
        };
        format!("Increases DNA capacity by {effect}")
    }

    fn description() -> &'static str {
        "Evolve Eukaryotic Cell"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            inc!(game.evolution.eukaryotic_cell);
            game.resources.dna.max += match game.evolution.mitochondria {
                Some(count) => count * 10 + 10,
                None => 10,
            } as f32;
        }
    }
}

//////////////////////////////////////////////

pub struct Mitochondria;
impl Structure for Mitochondria {
    const ID: &'static str = "mitochondria";
    const SIZE: usize = 2;

    fn title() -> &'static str {
        "Mitochondria"
    }

    fn cost(game: &Game) -> [Cost; Self::SIZE] {
        cost! {
            game,
            RNA => mitochondria, 75, 50,
            DNA => mitochondria, 65, 35
        }
    }

    fn effect(_: &Game) -> String {
        "Increases the effect of membranes and eukaryotic cells".to_string()
    }

    fn description() -> &'static str {
        "Evolve Mitochondria"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            inc!(game.evolution.mitochondria);
        }
    }
}

//////////////////////////////////////////////

pub struct SexualReproduction;
impl Structure for SexualReproduction {
    const ID: &'static str = "sexual_reproduction";
    const SIZE: usize = 1;

    fn title() -> &'static str {
        "Sexual Reproduction"
    }

    fn cost(_: &Game) -> [Cost; Self::SIZE] {
        cost!(DNA => 150)
    }

    fn effect(_: &Game) -> String {
        "Increases RNA generation from organelles".to_string()
    }

    fn description() -> &'static str {
        "Evolve Sexual Reproduction"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            assert!(game.evolution.sexual_reproduction.is_none());
            game.evolution.sexual_reproduction = Some(true);
            // TODO: only allow to be bought once

            // TODO: allow phagocytosis, chloroplasts, chitin to be purchased
        }
    }
}

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////

//////////////////////////////////////////////
