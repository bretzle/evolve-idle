use eframe::{App, CreationContext, Frame, NativeOptions};
use egui::{
    CentralPanel, Context, Grid, Layout, Sense, SidePanel, TextStyle, TopBottomPanel, Ui, Vec2,
    WidgetInfo, WidgetText, WidgetType,
};
use game::{Cost, GameData, GameStage};
use std::time::Instant;

mod building;
mod game;
mod util;

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
    rand: fastrand::Rng,
    last: Instant,
}

impl Application {
    pub fn new(_: &CreationContext<'_>) -> Self {
        let game = GameData::new();
        let rand = fastrand::Rng::with_seed(game.seed);

        Self {
            game,
            rand,
            last: Instant::now(),
        }
    }

    fn update(&mut self) {
        let secs_since_last_update = self.last.elapsed().as_secs_f64();
        self.last = Instant::now();

        let GameData {
            resource,
            evolution,
            stage,
            ..
        } = &mut self.game;

        match stage {
            GameStage::Evolution => {
                resource["RNA"] += evolution["organelles"] as f64 * secs_since_last_update;
                if resource["RNA"].amt.cur - (2 * evolution["nucleus"]) as f64 > 0.0 {
                    resource["RNA"] -= 2.0 * evolution["nucleus"] as f64 * secs_since_last_update;
                    resource["DNA"] += evolution["nucleus"] as f64 * secs_since_last_update;
                }
            }
            GameStage::Civilization => {}
        }
    }

    fn draw(&mut self, ctx: &Context, frame: &mut Frame) {
        let Self { game, rand, .. } = self;

        let width = ctx.available_rect().width();

        macro_rules! cost {
            ($cell:expr, $base:expr, $mult:expr) => {
                game.evolution[$cell] * $mult + $base
            };
        }

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
                            ui.label(*name);
                            ui.with_layout(Layout::right_to_left(), |ui| {
                                ui.label(format!(
                                    "{}/{}",
                                    res.amt.cur.floor(), // TODO: cool formating
                                    res.amt.max.floor(),
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
                    });
            });

        SidePanel::right("right_panel")
            .min_width(width / 4.0)
            .resizable(false)
            .show(ctx, |ui| {
                // Show message log + queue
            });

        CentralPanel::default().show(ctx, |ui| {
            match game.stage {
                GameStage::Evolution => {
                    ui.horizontal_wrapped(|ui| {
                        Action::new("RNA").build(ui, game, |game| {
                            game.resource["RNA"] += 1;
                        });

                        Action::new("Form DNA")
                            .add_cost("RNA", 2)
                            .build(ui, game, |game| {
                                game.resource["DNA"] += 1;
                            });

                        // Increase Max RNA by 5
                        Action::new("Membrane")
                            .add_cost("RNA", cost!("membrane", 2, 2))
                            .build(ui, game, |game| {
                                game.resource["RNA"].amt.max +=
                                    (game.evolution["mitochondria"] * 5 + 5) as f64;
                                game.evolution["membrane"] += 1;
                            });

                        // Automatically generate 1 RNA per second
                        Action::new("Organelles")
                            .add_cost("RNA", cost!("organelles", 12, 8))
                            .add_cost("DNA", cost!("organelles", 4, 4))
                            .build(ui, game, |game| {
                                game.evolution["organelles"] += 1;
                            });

                        // Automatically convert 2 RNA to 1 DNA per second
                        Action::new("Nucleus")
                            .add_cost("RNA", cost!("nucleus", 38, 32))
                            .add_cost("DNA", cost!("nucleus", 18, 16))
                            .build(ui, game, |game| {
                                game.evolution["nucleus"] += 1;
                            });

                        // Increase Max DNA by 10
                        Action::new("Eukaryotic Cell")
                            .add_cost("RNA", cost!("eukaryotic_cell", 20, 20))
                            .add_cost("DNA", cost!("eukaryotic_cell", 40, 12))
                            .build(ui, game, |game| {
                                game.evolution["eukaryotic_cell"] += 1;
                                game.resource["DNA"].amt.max +=
                                    (game.evolution["mitochondria"] * 10 + 10) as f64;
                            });

                        // Increase the effect of Membranes and Eukaryotic Cells
                        Action::new("Mitochondria")
                            .add_cost("RNA", cost!("mitochondria", 75, 50))
                            .add_cost("DNA", cost!("mitochondria", 65, 35))
                            .build(ui, game, |game| {
                                game.evolution["mitochondria"] += 1;
                                for _ in 0..game.evolution["membrane"] {
                                    game.resource["RNA"].amt.max += 5.0;
                                }
                                for _ in 0..game.evolution["eukaryotic_cell"] {
                                    game.resource["DNA"].amt.max += 10.0;
                                }
                            });

                        // Increase RNA generation from organelles. Unlocks evolution paths
                        Action::new("Sexual Reproduction")
                            .unlocked(!game.is_unlocked("sexual_reproduction"))
                            .add_cost("DNA", 150)
                            .build(ui, game, |game| {
                                game.evolution["sexual_reproduction"] += 1;
                                game.unlock("sexual_reproduction");

                                game.unlock("phagocytosis");
                                game.unlock("chloroplasts");
                                game.unlock("chitin");

                                // TODO: should there be an increment toward final progress?
                            });

                        // Evolve in the direction of the Animal Kingdom
                        Action::new("Phagocytosis")
                            .add_cost("DNA", 175)
                            .unlocked(game.is_unlocked("phagocytosis"))
                            .build(ui, game, |game| {
                                game.evolution["phagocytosis"] += 1;

                                game.lock("phagocytosis");
                                game.lock("chloroplasts");
                                game.lock("chitin");

                                game.unlock("multicellular");

                                // TODO: should there be an increment toward final progress?
                            });

                        // Evolve in the direction of the Plant Kingdom
                        Action::new("Chloroplasts")
                            .add_cost("DNA", 175)
                            .unlocked(game.is_unlocked("chloroplasts"))
                            .build(ui, game, |game| {
                                game.evolution["chloroplasts"] += 1;

                                game.lock("phagocytosis");
                                game.lock("chloroplasts");
                                game.lock("chitin");

                                game.unlock("multicellular");

                                // TODO: should there be an increment toward final progress?
                            });

                        // Evolve in the direction of the Fungi Kingdom
                        Action::new("Chitin")
                            .add_cost("DNA", 175)
                            .unlocked(game.is_unlocked("chitin"))
                            .build(ui, game, |game| {
                                game.evolution["chitin"] += 1;

                                game.lock("phagocytosis");
                                game.lock("chloroplasts");
                                game.lock("chitin");

                                game.unlock("multicellular");

                                // TODO: should there be an increment toward final progress?
                            });

                        // Become multicellular. Decrease cost of producing new nucleus
                        Action::new("Multicellular")
                            .unlocked(game.is_unlocked("multicellular"))
                            .add_cost("DNA", 200)
                            .build(ui, game, |game| {
                                game.evolution["multicellular"] += 1;
                                game.lock("multicellular");

                                if game.evolution.contains_key("phagocytosis") {
                                    todo!()
                                } else if game.evolution.contains_key("chloroplasts") {
                                    game.unlock("pokilohydric");
                                } else if game.evolution.contains_key("chitin") {
                                    todo!()
                                } else {
                                    unreachable!()
                                }

                                // TODO: should there be an increment toward final progress?
                            });

                        // Evolve Poikilohydric. Increase DNA generation from nucleus
                        Action::new("Poikilohydric")
                            .unlocked(game.is_unlocked("pokilohydric"))
                            .add_cost("DNA", 230)
                            .build(ui, game, |game| {
                                game.evolution["pokilohydric"] += 1;
                                game.lock("pokilohydric");
                                game.unlock("bryophyte");

                                // TODO: should there be an increment toward final progress?
                            });

                        // Evolve Bryophyte. Continue evolving towards sentience
                        Action::new("Bryophyte")
                            .unlocked(game.is_unlocked("bryophyte"))
                            .add_cost("DNA", 260)
                            .build(ui, game, |game| {
                                game.evolution["bryophyte"] += 1;
                                game.lock("bryophyte");
                                game.unlock("sentience");

                                println!("TODO: Unlock Entish, Cacti, Pinguicula");
                                // TODO: should there be an increment toward final progress?
                            });

                        // Complete your evolution
                        Action::new("Sentience")
                            .unlocked(game.is_unlocked("sentience"))
                            .add_cost("RNA", 300)
                            .add_cost("DNA", 300)
                            .build(ui, game, |game| {
                                game.evolution["sentience"] += 1;
                                game.lock("sentience");

                                let mut races = vec![];

                                if game.evolution.contains_key("chloroplasts") {
                                    races.extend(["entish", "cacti", "pinguicula"]);
                                } else {
                                    todo!()
                                }

                                game.race.species = races[rand.usize(0..races.len())];
                                // TODO: Check that player hasn't already played as that species

                                // TODO: Enter next game stage!
                                game.enter_sentience();
                            });
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

struct Action {
    label: &'static str,
    unlocked: bool,
    cost: Vec<Cost>,
}

impl Action {
    pub fn new(label: &'static str) -> Self {
        Self {
            label,
            unlocked: true,
            cost: vec![],
        }
    }

    pub fn unlocked(mut self, unlocked: bool) -> Self {
        self.unlocked = unlocked;
        self
    }

    pub fn add_cost<T: Into<f64>>(mut self, resource: &'static str, amount: T) -> Self {
        self.cost.push(Cost {
            resource,
            amount: amount.into(),
        });
        self
    }

    pub fn build<F>(self, ui: &mut Ui, game: &mut GameData, action: F)
    where
        F: FnOnce(&mut GameData),
    {
        if self.unlocked && game.afford(&self.cost) {
            let text: WidgetText = self.label.into();
            let frame = ui.visuals().button_frame;

            let button_padding = ui.spacing().button_padding;

            let total_extra = button_padding + button_padding;

            let wrap_width = ui.available_width() - total_extra.x;
            let text = text.into_galley(ui, None, wrap_width, TextStyle::Button);

            let desired_size = Vec2 {
                x: ui.available_width() / 4.0 - total_extra.x,
                y: 40.0,
            };

            let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
            response.widget_info(|| WidgetInfo::labeled(WidgetType::Button, text.text()));

            if ui.is_rect_visible(rect) {
                let visuals = ui.style().interact(&response);
                let text_pos = {
                    ui.layout()
                        .align_size_within_rect(text.size(), rect.shrink2(button_padding))
                        .min
                };

                if frame {
                    let fill = visuals.bg_fill;
                    let stroke = visuals.bg_stroke;
                    ui.painter().rect(
                        rect.expand(visuals.expansion),
                        visuals.rounding,
                        fill,
                        stroke,
                    );
                }

                text.paint_with_visuals(ui.painter(), text_pos, visuals);
            }

            if response.clicked() {
                game.pay(&self.cost);
                action(game);
            }
        }
    }
}
