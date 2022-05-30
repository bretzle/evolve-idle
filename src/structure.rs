use egui::Ui;

use crate::{
    button::Button,
    game::{Cost, GameData},
};

pub trait Structure {
    const ID: &'static str;
    const SIZE: usize;

    fn title() -> &'static str;
    fn cost(game: &GameData) -> [Cost; Self::SIZE];
    fn effect() -> &'static str;
    fn description() -> &'static str;
    fn action(game: &mut GameData);
}

#[inline(always)]
pub fn construct<T: Structure>(ui: &mut Ui, game: &mut GameData)
where
    [(); T::SIZE]:,
{
    let mut but = Button::new(T::title()).unlocked(game.is_unlocked(T::ID));

    for cost in T::cost(game) {
        but = but.add_cost(cost);
    }

    but.build::<T>(ui, game);
}
