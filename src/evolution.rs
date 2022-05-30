use crate::{
    game::{Cost, GameData},
    structure::Structure,
    RAND,
};

macro_rules! cost {
    ( $game:expr, $($resource:literal => $cell:literal, $base:expr, $mult:expr),* ) => {
        [$(
            Cost { resource: $resource, amount: ($game.evolution[$cell] * $mult + $base) as _ }
        ),*]
    };

    ($( $resource:literal => $amt:expr ),* ) => {
        [$( Cost { resource:$resource, amount:$amt } ),*]
    }
}

pub struct Rna;
impl Structure for Rna {
    const ID: &'static str = "rna";
    const SIZE: usize = 0;

    fn title() -> &'static str {
        "RNA"
    }

    fn cost(_: &GameData) -> [Cost; Self::SIZE] {
        []
    }

    fn effect() -> &'static str {
        "Creates 1 RNA"
    }

    fn description() -> &'static str {
        "Form new RNA"
    }

    fn action(game: &mut GameData) {
        if !game.resource["RNA"].is_full() {
            game.mod_res("RNA", 1.0, true, false);
        }
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

    fn cost(_: &GameData) -> [Cost; Self::SIZE] {
        [Cost {
            resource: "RNA",
            amount: 2.0,
        }]
    }

    fn effect() -> &'static str {
        "Turn 2 RNA into 1 DNA"
    }

    fn description() -> &'static str {
        "TODO"
    }

    fn action(game: &mut GameData) {
        if game.resource["RNA"].amount >= 2.0 && !game.resource["DNA"].is_full() {
            game.mod_res("RNA", -2.0, true, false);
            game.mod_res("DNA", 1.0, true, false);
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

    fn cost(game: &GameData) -> [Cost; Self::SIZE] {
        cost! {
            game,
            "RNA" => "membrane", 2, 2
        }
    }

    fn effect() -> &'static str {
        "Increases RNA capacity by 5" // TODO: this should be dynamic
    }

    fn description() -> &'static str {
        "Evolve Membranes"
    }

    fn action(game: &mut GameData) {
        game.resource["RNA"].max += (game.evolution["mitochondria"] * 5 + 5) as f32;
        game.evolution["membrane"] += 1;
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

    fn cost(game: &GameData) -> [Cost; Self::SIZE] {
        cost! {
            game,
            "RNA" => "organelles", 12, 8,
            "DNA" => "organelles", 4, 4
        }
    }

    fn effect() -> &'static str {
        "Automatically generate 1 RNA"
    }

    fn description() -> &'static str {
        "Evolve Organelles"
    }

    fn action(game: &mut GameData) {
        game.evolution["organelles"] += 1;
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

    fn cost(game: &GameData) -> [Cost; Self::SIZE] {
        cost! {
            game,
            "RNA" => "nucleus", 38, 32,
            "DNA" => "nucleus", 18, 16
        }
    }

    fn effect() -> &'static str {
        "automatically consume 2 RNA to create 1 DNA"
    }

    fn description() -> &'static str {
        "Evolve Nucleus"
    }

    fn action(game: &mut GameData) {
        game.evolution["nucleus"] += 1;
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

    fn cost(game: &GameData) -> [Cost; Self::SIZE] {
        cost! {
            game,
            "RNA" => "eukaryotic_cell", 20, 20,
            "DNA" => "eukaryotic_cell", 40, 12
        }
    }

    fn effect() -> &'static str {
        "Increase DNA capacity by 10"
    }

    fn description() -> &'static str {
        todo!()
    }

    fn action(game: &mut GameData) {
        game.evolution["eukaryotic_cell"] += 1;
        game.resource["DNA"].max += (game.evolution["mitochondria"] * 10 + 10) as f32;
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

    fn cost(game: &GameData) -> [Cost; Self::SIZE] {
        cost! {
            game,
            "RNA" => "mitochondria", 75, 50,
            "DNA" => "mitochondria", 65, 35
        }
    }

    fn effect() -> &'static str {
        "Increases the effect of membranes and eukaryotic cells"
    }

    fn description() -> &'static str {
        todo!()
    }

    fn action(game: &mut GameData) {
        game.evolution["mitochondria"] += 1;
        for _ in 0..game.evolution["membrane"] {
            game.resource["RNA"].max += 5.0;
        }
        for _ in 0..game.evolution["eukaryotic_cell"] {
            game.resource["DNA"].max += 10.0;
        }
        if game.evolution["sexual_reproduction"] == 0 {
            game.unlock("sexual_reproduction");
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

    fn cost(_: &GameData) -> [Cost; Self::SIZE] {
        cost!("DNA" => 150.0)
    }

    fn effect() -> &'static str {
        "Increases RNA generation from organelles"
    }

    fn description() -> &'static str {
        todo!()
    }

    fn action(game: &mut GameData) {
        game.evolution["sexual_reproduction"] += 1;
        game.lock("sexual_reproduction");

        game.unlock("phagocytosis");
        game.unlock("chloroplasts");
        game.unlock("chitin");

        // TODO: should there be an increment toward final progress?
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

    fn cost(_: &GameData) -> [Cost; Self::SIZE] {
        cost!("DNA" => 175.0)
    }

    fn effect() -> &'static str {
        "Evolve in the direction of the plant kingdom. This is a major evolutionary fork."
    }

    fn description() -> &'static str {
        todo!()
    }

    fn action(game: &mut GameData) {
        game.evolution["chloroplasts"] += 1;

        game.lock("phagocytosis");
        game.lock("chloroplasts");
        game.lock("chitin");

        game.unlock("multicellular");

        // TODO: should there be an increment toward final progress?
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

    fn cost(_: &GameData) -> [Cost; Self::SIZE] {
        cost!("DNA" => 200.0)
    }

    fn effect() -> &'static str {
        "Decreases cost of producing new nucleus."
    }

    fn description() -> &'static str {
        todo!()
    }

    fn action(game: &mut GameData) {
        game.evolution["multicellular"] += 1;
        game.lock("multicellular");

        if game.evolution.contains_key("phagocytosis") {
            todo!()
        } else if game.evolution.contains_key("chloroplasts") {
            game.unlock("poikilohydric");
        } else if game.evolution.contains_key("chitin") {
            todo!()
        } else {
            unreachable!()
        }

        // TODO: should there be an increment toward final progress?
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

    fn cost(_: &GameData) -> [Cost; Self::SIZE] {
        cost!("DNA" => 230.0)
    }

    fn effect() -> &'static str {
        "Increases DNA generation from nucleus"
    }

    fn description() -> &'static str {
        todo!()
    }

    fn action(game: &mut GameData) {
        game.evolution["poikilohydric"] += 1;
        game.lock("poikilohydric");
        game.unlock("bryophyte");

        // TODO: should there be an increment toward final progress?
    }
}

pub struct Bryophyte;
impl Structure for Bryophyte {
    const ID: &'static str = "bryophyte";
    const SIZE: usize = 1;

    fn title() -> &'static str {
        "Bryophyte"
    }

    fn cost(_: &GameData) -> [Cost; Self::SIZE] {
        cost!("DNA" => 260.0)
    }

    fn effect() -> &'static str {
        "Continue evolving towards sentience"
    }

    fn description() -> &'static str {
        todo!()
    }

    fn action(game: &mut GameData) {
        game.evolution["bryophyte"] += 1;
        game.lock("bryophyte");
        game.unlock("sentience");

        println!("TODO: Unlock Entish, Cacti, Pinguicula");
        // TODO: should there be an increment toward final progress?
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

    fn cost(_: &GameData) -> [Cost; Self::SIZE] {
        cost! {
            "RNA" => 300.0,
            "DNA" => 300.0
        }
    }

    fn effect() -> &'static str {
        "Complete your evolution by evolving into a species which has achieved sentience."
    }

    fn description() -> &'static str {
        todo!()
    }

    fn action(game: &mut GameData) {
        game.evolution["sentience"] += 1;
        game.lock("sentience");

        let mut races = vec![];

        if game.evolution.contains_key("chloroplasts") {
            races.extend(["entish", "cacti", "pinguicula"]);
        } else {
            todo!()
        }

        game.race.species = races[RAND.usize(0..races.len())];
        // TODO: Check that player hasn't already played as that species

        // TODO: Enter next game stage!
        game.enter_sentience();
    }
}
