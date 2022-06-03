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
use serde::{Deserialize, Serialize};
use std::{time::Duration, fs::File};
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
#[derive(Serialize, Deserialize)]
struct Game {
    seed: u64,
    resources: Resources,
    evolution: Evolution,
    tech: (),
    city: (),
    civic: (),
    race: Race,

    #[serde(skip)]
    rng: Rng,

    // Ui stuff
	#[serde(skip)]
    actions: usize,
}

impl Game {
    pub fn new(engine: &mut Engine) -> Self {
        let Engine { clockwork, .. } = engine;

        clockwork.every(Duration::from_millis(250)).run(Game::fast_loop);
        clockwork.every(Duration::from_millis(1000)).run(Game::mid_loop);
        clockwork.every(Duration::from_millis(5000)).run(Game::long_loop);

        Self::load_save().unwrap_or_else(|| Self {
            seed: 1,
            resources: Resources::new(),
            evolution: Evolution::new(),
            tech: (),
            city: (),
            civic: (),
            race: Race::default(),

            rng: Rng::with_seed(1),
            actions: 0,
        })
    }

    fn load_save() -> Option<Self> {
        let content = match std::fs::read_to_string("save.json") {
            Ok(content) => content,
            Err(_) => return None,
        };

        serde_json::from_str(&content).ok()
    }

	fn save(&self) {
		let x = serde_json::to_vec_pretty(self).unwrap();
		let mut file = File::create("save.json").unwrap();
		use std::io::Write;
		file.write(&x).unwrap();
	}

	fn on_exit(&self) {
		self.save();
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
                if self.evolution.sexual_reproduction > 0 {
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
            } else if evolution.mitochondria >= 1 && evolution.sexual_reproduction == -1 {
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
            let base = 100.0;
            let mut rna_cap = base;
            let mut dna_cap = base;
            if self.evolution.membrane != -1 {
                let effect = if self.evolution.mitochondria != -1 {
                    self.evolution.mitochondria * 5 + 5
                } else {
                    5
                };
                rna_cap += (self.evolution.membrane * effect) as f32;
            }
            if self.evolution.eukaryotic_cell != -1 {
                let effect = if self.evolution.mitochondria != -1 {
                    self.evolution.mitochondria * 10 + 10
                } else {
                    10
                };
                dna_cap += (self.evolution.eukaryotic_cell * effect) as f32;
            }

            self.resources.rna.max = rna_cap;
            self.resources.dna.max = dna_cap;
        } else {
            todo!()
        }
    }

    // Runs every 5 seconds
    fn long_loop(&mut self) {
        // autosave
		self.save();
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
                let size = ui.content_region_avail();
                if let Some(_) = ui.begin_table_with_sizing("res table", 3, TableFlags::ROW_BG, size, 0.0) {
                    all::<ResourceType>().for_each(|res| {
                        let resource = &self.resources[res];
                        if resource.display {
                            ui.table_next_column();
                            ui.text(format!("{res}"));
                            ui.table_next_column();
                            util::right_align(ui, format!("{}/{}", resource.amount.floor(), resource.max));
                            ui.table_next_column();
                            util::right_align(ui, format!("{} /s", resource.diff));
                        }
                    });
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
                        self.actions = 0;

                        construct!(evolution::Rna, ui, self);
                        construct!(evolution::Dna, ui, self);
                        construct!(evolution::Membrane, ui, self.evolution.membrane);
                        construct!(evolution::Organelles, ui, self.evolution.organelles);
                        construct!(evolution::Nucleus, ui, self.evolution.nucleus);
                        construct!(evolution::EukaryoticCell, ui, self.evolution.eukaryotic_cell);
                        construct!(evolution::Mitochondria, ui, self.evolution.mitochondria);
                        construct!(evolution::SexualReproduction, ui, self.evolution.sexual_reproduction);

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
