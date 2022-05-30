#![feature(generic_const_exprs)]
#![allow(incomplete_features)]
#![warn(clippy::all)]

use eframe::{App, CreationContext, Frame, NativeOptions};
use egui::{CentralPanel, Context, Grid, Layout, SidePanel, TopBottomPanel};
use game::{GameData, GameStage};
use std::time::Instant;
use util::Rng;

mod button;
mod evolution;
mod game;
mod structure;
mod util;

pub static RAND: Rng = Rng::new();

fn main() {
    let native_options = NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(Application::new(cc))),
    );
}

pub struct Application {
    game: GameData,
    last: Instant,
}

impl Application {
    pub fn new(_: &CreationContext<'_>) -> Self {
        let game = GameData::new();
        RAND.seed(game.seed);

        Self {
            game,
            last: Instant::now(),
        }
    }

    fn update(&mut self) {
        let secs_since_last_update = self.last.elapsed().as_secs_f64();
        self.last = Instant::now();

        let game = &mut self.game;

        match game.stage {
            GameStage::Evolution => {
                // Gain DNA
                if game.evolution["nucleus"] > 0 && !game.resource["DNA"].is_full() {
                    todo!()
                }

                // Gain RNA
                if game.evolution["organelles"] > 0 {
                    let mut rna_multiplier = 1;
                    if game.evolution["sexual_reproduction"] > 0 {
                        rna_multiplier += 1;
                    }
                    game.mod_res(
                        "RNA",
                        game.evolution["organelles"] * rna_multiplier,
                        false,
                        false,
                    );
                }

                // Detect new unlocks
                if game.resource["RNA"].amount >= 2.0 && !game.is_unlocked("dna") {
                    game.unlock("dna");
                    game.resource["DNA"].display = true;
                } else if game.resource["RNA"].amount >= 10.0 && !game.is_unlocked("membrane") {
                    game.unlock("membrane");
                } else if game.resource["DNA"].amount >= 4.0 && !game.is_unlocked("organelles") {
                    game.unlock("organelles");
                } else if game.evolution["organelles"] >= 2 && !game.is_unlocked("nucleus") {
                    game.unlock("nucleus");
                } else if game.evolution["nucleus"] >= 1 && !game.is_unlocked("eukaryotic_cell") {
                    game.unlock("eukaryotic_cell");
                } else if game.evolution["eukaryotic_cell"] >= 1
                    && !game.is_unlocked("mitochondria")
                {
                    game.unlock("mitochondria");
                } else if game.evolution["mitochondria"] >= 1
                    && !game.is_unlocked("sexual_reproduction")
                {
                    game.unlock("sexual_reproduction");
                }
            }
            GameStage::Civilization => {}
        }
    }

    fn draw(&mut self, ctx: &Context, _frame: &mut Frame) {
        let Self { game, .. } = self;
        let width = ctx.available_rect().width();

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.label("MOON PHASE");
                ui.label("DATE");
                ui.label("WEATHER");
                ui.label("PLAY/PAUSE");
            });
        });

        TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.label("MOON PHASE");
                ui.label("DATE");
                ui.label("WEATHER");
                ui.label("PLAY/PAUSE");
            });
        });

        SidePanel::left("resource_panel")
            .min_width(width / 4.0)
            .resizable(false)
            .show(ctx, |ui| {
                Grid::new("resource-grid")
                    .num_columns(3)
                    .striped(true)
                    .min_col_width(width / 4.0 / 3.0)
                    .show(ui, |ui| {
                        for (name, res) in game.resource.iter() {
                            if res.display {
                                ui.label(*name);
                                ui.with_layout(Layout::right_to_left(), |ui| {
                                    ui.label(format!(
                                        "{}/{}",
                                        res.amount.floor(), // TODO: cool formating
                                        res.max.floor(),
                                    ));
                                });
                                ui.with_layout(
                                    Layout::from_main_dir_and_cross_align(
                                        egui::Direction::RightToLeft,
                                        egui::Align::Max,
                                    ),
                                    |ui| {
                                        ui.label("? /s");
                                    },
                                );
                                ui.end_row();
                            }
                        }
                    });
            });

        SidePanel::right("right_panel")
            .min_width(width / 4.0)
            .resizable(false)
            .show(ctx, |_| {
                // Show message log + queue
            });

        CentralPanel::default().show(ctx, |ui| {
            match game.stage {
                GameStage::Evolution => {
                    ui.horizontal_wrapped(|ui| {
                        use structure::construct;

                        construct::<evolution::Rna>(ui, game);
                        construct::<evolution::Dna>(ui, game);
                        construct::<evolution::Membrane>(ui, game);
                        construct::<evolution::Organelles>(ui, game);
                        construct::<evolution::Nucleus>(ui, game);
                        construct::<evolution::EukaryoticCell>(ui, game);
                        construct::<evolution::Mitochondria>(ui, game);
                        construct::<evolution::SexualReproduction>(ui, game);

                        // construct::<evolution::Phagocytosis>(ui, game);
                        construct::<evolution::Chloroplasts>(ui, game);
                        // construct::<evolution::Chitin>(ui, game);

                        construct::<evolution::Multicellular>(ui, game);
                        construct::<evolution::Poikilohydric>(ui, game);
                        construct::<evolution::Bryophyte>(ui, game);

                        construct::<evolution::Sentience>(ui, game);
                    });
                }
                GameStage::Civilization => {}
            }
        });
    }
}

impl App for Application {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        self.update();
        self.draw(ctx, frame);

        ctx.request_repaint();
    }
}
