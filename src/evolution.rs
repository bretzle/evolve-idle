use crate::{
    race::Species,
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

    pub(crate) fn is_unlocked(&self, id: &str) -> bool {
        match id {
            "rna" => true,
            "dna" => self.dna_unlocked,
            "membrane" => self.membrane.is_some(),
            "organelles" => self.organelles.is_some(),
            "nucleus" => self.nucleus.is_some(),
            "eukaryotic_cell" => self.eukaryotic_cell.is_some(),
            "mitochondria" => self.mitochondria.is_some(),
            "sexual_reproduction" => self.sexual_reproduction == Some(false),
            "phagocytosis" => self.phagocytosis == Some(false),
            "chloroplasts" => self.chloroplasts == Some(false),
            "chitin" => self.chitin == Some(false),
            "multicellular" => self.multicellular == Some(false),
			"bilateral_symmetry" => self.bilateral_symmetry == Some(false),
			"poikilohydric" => self.poikilohydric == Some(false),
			"spores" => self.spores == Some(false),
			"bryophyte" => self.bryophyte == Some(false),
			"sentience" => self.sentience == Some(false),
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
        assert!(game.evolution.sexual_reproduction == Some(false));
        if pay::<Self>(game) {
            game.evolution.sexual_reproduction = Some(true);

            game.evolution.phagocytosis = Some(false);
            game.evolution.chloroplasts = Some(false);
            game.evolution.chitin = Some(false);

            game.evolution.progress = Some(20);
        }
    }
}

//////////////////////////////////////////////

pub struct Phagocytosis;
impl Structure for Phagocytosis {
    const ID: &'static str = "phagocytosis";
    const SIZE: usize = 1;

    fn title() -> &'static str {
        "Phagocytosis"
    }

    fn cost(_: &Game) -> [Cost; Self::SIZE] {
        cost!(DNA => 175)
    }

    fn effect(_: &Game) -> String {
        "Evolve in the direction of the animal kingdom. This is a major evolutionary fork.".to_string()
    }

    fn description() -> &'static str {
        // "Evolve Phagocytosis"
        "This path is yet to be developed. Do not purchase it."
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            game.evolution.phagocytosis = Some(true);
            game.evolution.chloroplasts = None;
            game.evolution.chitin = None;
            game.evolution.multicellular = Some(false);
            game.evolution.progress = Some(40);
        }
    }
}

//////////////////////////////////////////////

pub struct Chloroplasts;
impl Structure for Chloroplasts {
    const ID: &'static str = "chloroplasts";
    const SIZE: usize = 1;

    fn title() -> &'static str {
        "Chloroplasts"
    }

    fn cost(_: &Game) -> [Cost; Self::SIZE] {
        cost!(DNA => 175)
    }

    fn effect(_: &Game) -> String {
        "Evolve in the direction of the plant kingdom. This is a major evolutionary fork.".to_string()
    }

    fn description() -> &'static str {
        "Evolve Chloroplasts"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            game.evolution.chloroplasts = Some(true);
            game.evolution.phagocytosis = None;
            game.evolution.chitin = None;
            game.evolution.multicellular = Some(false);
            game.evolution.progress = Some(40);
        }
    }
}

//////////////////////////////////////////////

pub struct Chitin;
impl Structure for Chitin {
    const ID: &'static str = "chitin";
    const SIZE: usize = 1;

    fn title() -> &'static str {
        "Chitin"
    }

    fn cost(_: &Game) -> [Cost; Self::SIZE] {
        cost!(DNA => 175)
    }

    fn effect(_: &Game) -> String {
        "Evolve in the direction of the fungi kingdom. This is a major evolutionary fork.".to_string()
    }

    fn description() -> &'static str {
        "Evolve Chitin"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            game.evolution.chitin = Some(true);
            game.evolution.phagocytosis = None;
            game.evolution.chloroplasts = None;
            game.evolution.multicellular = Some(false);
            game.evolution.progress = Some(40);
        }
    }
}

//////////////////////////////////////////////

pub struct Multicellular;
impl Structure for Multicellular {
    const ID: &'static str = "multicellular";
    const SIZE: usize = 1;

    fn title() -> &'static str {
        "Multicellular"
    }

    fn cost(_: &Game) -> [Cost; Self::SIZE] {
        cost!(DNA => 200)
    }

    fn effect(_: &Game) -> String {
        "Decreases cost of producing new nucleus.".to_string()
    }

    fn description() -> &'static str {
        "Evolve Multicellular"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            game.evolution.multicellular = Some(true);
            game.evolution.progress = Some(60);

            if game.evolution.phagocytosis.is_some() {
                game.evolution.bilateral_symmetry = Some(false);
            } else if game.evolution.chloroplasts.is_some() {
                game.evolution.poikilohydric = Some(false);
            } else if game.evolution.chitin.is_some() {
                game.evolution.spores = Some(false);
            }
        }
    }
}

//////////////////////////////////////////////

pub struct Spores;
impl Structure for Spores {
    const ID: &'static str = "spores";
    const SIZE: usize = 1;

    fn title() -> &'static str {
        "Spores"
    }

    fn cost(_: &Game) -> [Cost; Self::SIZE] {
        cost!(DNA => 230)
    }

    fn effect(_: &Game) -> String {
        "Increases DNA generation from nucleus".to_string()
    }

    fn description() -> &'static str {
        "Evolve Spores"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            game.evolution.spores = Some(true);
            game.evolution.bryophyte = Some(false);
            game.evolution.progress = Some(80);
        }
    }
}

//////////////////////////////////////////////

pub struct Poikilohydric;
impl Structure for Poikilohydric {
    const ID: &'static str = "poikilohydric";
    const SIZE: usize = 1;

    fn title() -> &'static str {
        "Poikilohydric"
    }

    fn cost(_: &Game) -> [Cost; Self::SIZE] {
        cost!(DNA => 230)
    }

    fn effect(_: &Game) -> String {
        "Increases DNA generation from nucleus".to_string()
    }

    fn description() -> &'static str {
        "Evolve Poikilohydric"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            game.evolution.poikilohydric = Some(true);
            game.evolution.bryophyte = Some(false);
            game.evolution.progress = Some(80);
        }
    }
}

//////////////////////////////////////////////

pub struct BilateralSymmetry;
impl Structure for BilateralSymmetry {
    const ID: &'static str = "bilateral_symmetry";
    const SIZE: usize = 1;

    fn title() -> &'static str {
        "BilateralSymmetry"
    }

    fn cost(_: &Game) -> [Cost; Self::SIZE] {
        cost!(DNA => 230)
    }

    fn effect(_: &Game) -> String {
        "Increases DNA generation from nucleus".to_string()
    }

    fn description() -> &'static str {
        "This path is yet to be developed. Do not purchase it."
    }

    fn action(_: &mut Game) {
        println!("Animal kingdom is not implemented yet")
    }
}

//////////////////////////////////////////////

pub struct Bryophyte;
impl Structure for Bryophyte {
    const ID: &'static str = "bryophyte";
    const SIZE: usize = 1;

    fn title() -> &'static str {
        "Bryophyte"
    }

    fn cost(_: &Game) -> [Cost; Self::SIZE] {
        cost!(DNA => 260)
    }

    fn effect(_: &Game) -> String {
        "Continue evolving towards sentience".to_string()
    }

    fn description() -> &'static str {
        "Evolve Bryophyte"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            game.evolution.bryophyte = Some(true);
            game.evolution.progress = Some(100);
            game.evolution.sentience = Some(false);
        }
    }
}

//////////////////////////////////////////////

pub struct Sentience;
impl Structure for Sentience {
    const ID: &'static str = "sentience";
    const SIZE: usize = 2;

    fn title() -> &'static str {
        "Sentience"
    }

    fn cost(_: &Game) -> [Cost; Self::SIZE] {
        cost! {
            RNA => 300,
            DNA => 300
        }
    }

    fn effect(_: &Game) -> String {
        "Complete your evolution by evolving into a species which has achieved sentience.".to_string()
    }

    fn description() -> &'static str {
        "Evolve Sentience"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
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
