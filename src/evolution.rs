use crate::{
    resource::ResourceType::*,
    structure::{Cost, Structure},
    Game,
};

macro_rules! cost {
    ( $game:expr, $($resource:ident => $cell:tt, $base:expr, $mult:expr),* ) => {
        [$(
            Cost { resource: $resource, amount: ($game.evolution.$cell * $mult + $base) as _ }
        ),*]
    };

    ($( $resource:ident => $amt:expr ),* ) => {
        [$( Cost { resource:$resource, amount:$amt as _ } ),*]
    }
}

pub struct Evolution {
    pub dna_unlocked: bool,
    pub membrane: i32,
    pub organelles: i32,
    pub nucleus: i32,
    pub eukaryotic_cell: i32,
    pub mitochondria: i32,
    pub sexual_reproduction: i32,
    pub multicellular: i32,
}

impl Evolution {
    pub fn new() -> Self {
        Self {
            dna_unlocked: false,
            membrane: -1,
            organelles: -1,
            nucleus: -1,
            eukaryotic_cell: -1,
            mitochondria: -1,
            sexual_reproduction: -1,
            multicellular: -1,
        }
    }

    pub(crate) fn is_unlocked(&self, id: &str) -> bool {
        match id {
            "rna" => true,
            "dna" => self.dna_unlocked,
            "membrane" => self.membrane != -1,
            "organelles" => self.organelles != -1,
            "nucleus" => self.nucleus != -1,
            "eukaryotic_cell" => self.eukaryotic_cell != -1,
            "mitochondria" => self.mitochondria != -1,
            "sexual_reproduction" => self.sexual_reproduction != -1 && self.sexual_reproduction != 1,
            "multicellular" => self.multicellular != -1,
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
        let effect = if game.evolution.mitochondria != -1 {
            game.evolution.mitochondria * 5 + 5
        } else {
            5
        };
        format!("Increases RNA capacity by {effect}")
    }

    fn description() -> &'static str {
        "Evolve Membranes"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            game.resources.rna.max += if game.evolution.mitochondria != -1 {
                game.evolution.mitochondria * 5 + 5
            } else {
                5
            } as f32;
            game.evolution.membrane += 1;
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
        if game.evolution.sexual_reproduction > 0 {
            rna += 1;
        }
        format!("Automatically generate {rna} RNA")
    }

    fn description() -> &'static str {
        "Evolve Organelles"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            game.evolution.organelles += 1;
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
        let multi = game.evolution.multicellular > 0;
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
            game.evolution.nucleus += 1;
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
        let effect = if game.evolution.mitochondria != -1 {
            game.evolution.mitochondria * 10 + 10
        } else {
            10
        };
        format!("Increases DNA capacity by {effect}")
    }

    fn description() -> &'static str {
        "Evolve Eukaryotic Cell"
    }

    fn action(game: &mut Game) {
        if pay::<Self>(game) {
            game.evolution.eukaryotic_cell += 1;
            let mitochondria = game.evolution.mitochondria;
            game.resources.dna.max += if mitochondria != -1 { mitochondria * 10 + 10 } else { 10 } as f32;
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
            game.evolution.mitochondria += 1;
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
            game.evolution.sexual_reproduction += 1;
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
