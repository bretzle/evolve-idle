#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![warn(clippy::all)]

use engine::Engine;
use enum_iterator::all;
use evolution::Evolution;
use fastrand::Rng;
use imgui::{TableFlags, Ui};
use race::{Race, Species};
use resource::{ResourceType, Resources};
use std::time::Duration;
use structure::Cost;

mod clockwork;
mod engine;
mod evolution;
mod race;
mod resource;
mod structure;
mod util;

const VERSION: &'static str = concat!("v", env!("CARGO_PKG_VERSION"));

fn main() {
    Engine::new("Evolve", [1024, 768]).run()
}

#[allow(dead_code)] // remove this
struct Game {
    seed: u64,
    resources: Resources,
    evolution: Evolution,
    tech: (),
    city: (),
    civic: (),
    race: Race,

    rng: Rng,

    // Ui stuff
    actions: usize,
}

impl Game {
    pub fn new(engine: &mut Engine) -> Self {
        let Engine { clockwork, .. } = engine;

        clockwork.every(Duration::from_millis(250)).run(Game::fast_loop);
        clockwork.every(Duration::from_millis(1000)).run(Game::mid_loop);
        clockwork.every(Duration::from_millis(5000)).run(Game::long_loop);

        Self {
            seed: 1,
            resources: Resources::new(),
            evolution: Evolution::new(),
            tech: (),
            city: (),
            civic: (),
            race: Race::default(),

            rng: Rng::with_seed(1),
            actions: 0,
        }
    }

    // Runs every 0.25 seconds
    fn fast_loop(&mut self) {
        let global_mult = 1;
        let time_mult = 0.25;
        if matches!(self.race.species, Species::Protoplasm) {
            use ResourceType::*;
            // Gain DNA
            if self.evolution.nucleus != -1 && !self.resources.dna.is_full() {
                let mut increment = self.evolution.nucleus;
                while self.resources.rna.amount < (increment * 2) as f32 {
                    increment -= 1;
                    if increment <= 0 {
                        break;
                    }
                }
                let rna = increment;
                // TODO: bilateral_symmetry, poikilohydric, spores should upgrade this

                self.mod_res(DNA, (increment * global_mult) as f32 * time_mult, false, false);
                self.mod_res(RNA, -((rna * 2) as f32 * time_mult), false, false);
            }

            // Gain RNA
            let organelles = self.evolution.organelles;
            if organelles != -1 {
                let mut mult = 1;
                if self.evolution.sexual_reproduction != -1 {
                    mult += 1;
                }

                self.mod_res(RNA, (organelles * mult * global_mult) as f32 * time_mult, false, false);
            }

            // Detect new unlocks
            let Self {
                resources, evolution, ..
            } = self;

            if resources.rna.amount >= 2.0 && !evolution.dna_unlocked {
                evolution.dna_unlocked = true;
                resources.dna.display = true;
            } else if resources.rna.amount >= 10.0 && !evolution.is_unlocked("membrane") {
                evolution.membrane = 0;
            } else if resources.dna.amount >= 2.0 && !evolution.is_unlocked("organelles") {
                evolution.organelles = 0;
            } else if evolution.organelles >= 2 && !evolution.is_unlocked("nucleus") {
                evolution.nucleus = 0;
            } else if evolution.nucleus >= 1 && !evolution.is_unlocked("eukaryotic_cell") {
                evolution.eukaryotic_cell = 0;
            } else if evolution.eukaryotic_cell >= 1 && !evolution.is_unlocked("mitochondria") {
                evolution.mitochondria = 0;
            } else if evolution.mitochondria >= 1 && !evolution.is_unlocked("sexual_reproduction") {
                evolution.sexual_reproduction = 0;
            }
        } else {
            todo!()
        }

        // main resource tracking
        for res in all::<ResourceType>() {
            let resource = &self.resources[res];
            if resource.rate > 0.0 || (resource.rate == 0.0 && resource.max == -1.0) {
                self.diff_calc(res, 250.0)
            }
        }
    }

    // Runs every 1 second
    fn mid_loop(&mut self) {
        // update resource caps
        if matches!(self.race.species, Species::Protoplasm) {
            //
        } else {
            todo!()
        }
    }

    // Runs every 5 seconds
    fn long_loop(&mut self) {
        // autosave
    }

    pub fn update(&mut self, ui: &mut Ui) {
        ui.main_menu_bar(|| {
            ui.text("Prehistoric");
            util::right_align(ui, VERSION);
        });
        util::statusbar(|| ui.text("Evolve by John"));

        let (width, height, pos) = unsafe {
            let viewport = *imgui::sys::igGetMainViewport();
            let size = viewport.WorkSize;
            let pos = viewport.WorkPos;
            let offset = viewport.Size.y - viewport.WorkSize.y;
            (size.x, size.y - offset, pos)
        };

        ui.window("left panel")
            .size([width / 4.0, height], imgui::Condition::Always)
            .position([pos.x, pos.y], imgui::Condition::Always)
            .focused(false)
            .title_bar(false)
            .movable(false)
            .resizable(false)
            .draw_background(false)
            .build(|| {
                // ui.text(format!("RNA: {}", self.resources.rna.amount));
                // ui.text(format!("DNA: {}", self.resources.dna.amount));

                let size = ui.content_region_avail();
                if let Some(_) = ui.begin_table_with_sizing("res table", 3, TableFlags::ROW_BG, size, 0.0) {
                    ui.table_next_column();
                    ui.text("RNA");
                    ui.table_next_column();
                    util::right_align(
                        ui,
                        format!("{}/{}", self.resources.rna.amount.floor(), self.resources.rna.max),
                    );
                    ui.table_next_column();
                    util::right_align(ui, format!("{} /s", self.resources.rna.diff));

                    ui.table_next_column();
                    ui.text("DNA");
                    ui.table_next_column();
                    util::right_align(
                        ui,
                        format!("{}/{}", self.resources.dna.amount.floor(), self.resources.dna.max),
                    );
                    ui.table_next_column();
                    util::right_align(ui, format!("{} /s", self.resources.dna.diff));
                }
            });

        ui.window("main panel")
            .size([width / 2.0, height], imgui::Condition::Always)
            .position([width / 4.0, pos.y], imgui::Condition::Always)
            .focused(false)
            .title_bar(false)
            .movable(false)
            .resizable(false)
            // .draw_background(false)
            .build(|| {
                if let Some(_tab) = ui.tab_bar("tabs") {
                    if let Some(_tab) = ui.tab_item("Evolve") {
                        use structure::construct;
                        self.actions = 0;

                        construct::<evolution::Rna>(ui, self);
                        construct::<evolution::Dna>(ui, self);
                        construct::<evolution::Membrane>(ui, self);
                        construct::<evolution::Organelles>(ui, self);
                        construct::<evolution::Nucleus>(ui, self);
                        construct::<evolution::EukaryoticCell>(ui, self);
                        construct::<evolution::Mitochondria>(ui, self);
                        construct::<evolution::SexualReproduction>(ui, self);

                        // // construct::<evolution::Phagocytosis>(ui, game);
                        // construct::<evolution::Chloroplasts>(ui, self);
                        // // construct::<evolution::Chitin>(ui, game);

                        // construct::<evolution::Multicellular>(ui, self);
                        // construct::<evolution::Poikilohydric>(ui, self);
                        // construct::<evolution::Bryophyte>(ui, self);

                        // construct::<evolution::Sentience>(ui, self);
                    }
                    if let Some(_tab) = ui.tab_item("Settings") {}
                }
            });

        ui.window("right panel")
            .size([width / 4.0, height], imgui::Condition::Always)
            .position([3.0 * width / 4.0, pos.y], imgui::Condition::Always)
            .focused(false)
            .title_bar(false)
            .movable(false)
            .resizable(false)
            .draw_background(false)
            .build(|| {});

        ui.show_demo_window(&mut true);
    }
}

impl Game {
    fn diff_calc(&mut self, res: ResourceType, period: f32) {
        let sec = 1000.0;

        self.resources[res].diff = self.resources[res].delta / (period / sec);
        self.resources[res].delta = 0.0;
    }

    pub(crate) fn afford(&self, costs: &[Cost]) -> bool {
        for Cost { resource, amount } in costs {
            if self.resources[*resource].amount < *amount {
                return false;
            }
        }

        true
    }

    pub(crate) fn mod_res(&mut self, res: ResourceType, val: f32, notrack: bool, buffer: bool) -> bool {
        let mut count = self.resources[res].amount + val;
        let mut success = true;

        if count > self.resources[res].max && self.resources[res].max != -1.0 {
            count = self.resources[res].max;
        } else if count < 0.0 {
            if !buffer || (buffer && (-count > buffer as u32 as f32)) {
                success = false;
            }
            count = 0.0;
        }

        if !count.is_nan() {
            self.resources[res].amount = count;
            if !notrack {
                self.resources[res].delta += val;
                // TODO: mana
            }
        }

        success
    }

    pub(crate) fn is_unlocked(&self, id: &str) -> bool {
        match id {
            "rna"
            | "dna"
            | "membrane"
            | "organelles"
            | "nucleus"
            | "eukaryotic_cell"
            | "mitochondria"
            | "sexual_reproduction"
            | "multicellular" => self.evolution.is_unlocked(id),
            _ => panic!("id: {id:?} does not exist."),
        }
    }

    pub fn check_costs(&self, costs: &[Cost]) -> bool {
        let mut test = true;
        for cost in costs {
            match cost.resource {
                // TODO: special cases
                res => {
                    let test_cost = cost.amount;
                    if test_cost == 0.0 {
                        break;
                    }
                    let fail_max = if self.resources[res].max >= 0.0 && test_cost > self.resources[res].max {
                        true
                    } else {
                        false
                    };
                    if test_cost > self.resources[res].amount + self.resources[res].diff || fail_max {
                        test = false;
                        break;
                    }
                }
            }
        }
        test
    }
}

// impl Application {

//     fn update(&mut self) {
//         let secs_since_last_update = self.last.elapsed().as_secs_f64();
//         self.last = Instant::now();

//         let game = &mut self.game;

//         match game.stage {
//             GameStage::Evolution => {
//                 // Gain DNA
//                 if game.evolution["nucleus"] > 0 && !game.resource["DNA"].is_full() {
//                     let mut increment = game.evolution["nucleus"] as i32;
//                     while game.resource["RNA"].amount < (increment * 2) as f32 {
//                         increment -= 1;
//                         if increment <= 0 {
//                             break;
//                         }
//                     }
//                     let rna = increment;
//                     if game.is_unlocked("poikilohydric") && game.evolution["poikilohydric"] > 0 {
//                         increment *= 2;
//                     }

//                     game.mod_res("DNA", increment as f32, false, false);
//                     game.mod_res("RNA", -(rna * 2) as f32, false, false);
//                 }

//                 // Gain RNA
//                 if game.evolution["organelles"] > 0 {
//                     let mut rna_multiplier = 1;
//                     if game.evolution["sexual_reproduction"] > 0 {
//                         rna_multiplier += 1;
//                     }
//                     game.mod_res(
//                         "RNA",
//                         game.evolution["organelles"] * rna_multiplier,
//                         false,
//                         false,
//                     );
//                 }

//                 // Detect new unlocks
//                 if game.resource["RNA"].amount >= 2.0 && !game.is_unlocked("dna") {
//                     game.unlock("dna");
//                     game.resource["DNA"].display = true;
//                 } else if game.resource["RNA"].amount >= 10.0 && !game.is_unlocked("membrane") {
//                     game.unlock("membrane");
//                 } else if game.resource["DNA"].amount >= 4.0 && !game.is_unlocked("organelles") {
//                     game.unlock("organelles");
//                 } else if game.evolution["organelles"] >= 2 && !game.is_unlocked("nucleus") {
//                     game.unlock("nucleus");
//                 } else if game.evolution["nucleus"] >= 1 && !game.is_unlocked("eukaryotic_cell") {
//                     game.unlock("eukaryotic_cell");
//                 } else if game.evolution["eukaryotic_cell"] >= 1
//                     && !game.is_unlocked("mitochondria")
//                 {
//                     game.unlock("mitochondria");
//                 }
//             }
//             GameStage::Civilization => {}
//         }
//     }

//     fn draw(&mut self, ctx: &Context, _frame: &mut Frame) {
//         let Self { game, .. } = self;
//         let width = ctx.available_rect().width();

//         CentralPanel::default().show(ctx, |ui| {
//             match game.stage {
//                 GameStage::Evolution => {
//                     ui.horizontal_wrapped(|ui| {
//                         use structure::construct;

//                         construct::<evolution::Rna>(ui, game);
//                         construct::<evolution::Dna>(ui, game);
//                         construct::<evolution::Membrane>(ui, game);
//                         construct::<evolution::Organelles>(ui, game);
//                         construct::<evolution::Nucleus>(ui, game);
//                         construct::<evolution::EukaryoticCell>(ui, game);
//                         construct::<evolution::Mitochondria>(ui, game);
//                         construct::<evolution::SexualReproduction>(ui, game);

//                         // construct::<evolution::Phagocytosis>(ui, game);
//                         construct::<evolution::Chloroplasts>(ui, game);
//                         // construct::<evolution::Chitin>(ui, game);

//                         construct::<evolution::Multicellular>(ui, game);
//                         construct::<evolution::Poikilohydric>(ui, game);
//                         construct::<evolution::Bryophyte>(ui, game);

//                         construct::<evolution::Sentience>(ui, game);
//                     });
//                 }
//                 GameStage::Civilization => {}
//             }
//         });
//     }
// }
