use crate::{resource::Cost, Game};
use imgui::Ui;

pub(crate) trait Structure {
    const ID: &'static str;
    const SIZE: usize;

    fn title() -> &'static str;
    fn cost(game: &Game) -> [Cost; Self::SIZE];
    fn effect(game: &Game) -> String;
    fn description() -> &'static str;
    fn action(game: &mut Game);

    fn tooltip(ui: &Ui, game: &Game)
    where
        [(); Self::SIZE]:,
    {
        ui.tooltip(|| {
            ui.text(Self::description());
            ui.separator();
            // ui.text("COST: todo");
            for cost in &Self::cost(game) {
                ui.text(format!("{}: {}", cost.resource, cost.amount))
            }
            ui.separator();
            ui.text(Self::effect(game));
        });
    }
}
