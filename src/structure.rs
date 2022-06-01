use crate::{resource::ResourceType, Game};
use imgui::Ui;

#[derive(Clone, Copy)]
pub struct Cost {
    pub resource: ResourceType,
    pub amount: f32,
}

pub(crate) trait Structure {
    const ID: &'static str;
    const SIZE: usize;

    fn title() -> &'static str;
    fn cost(game: &Game) -> [Cost; Self::SIZE];
    fn effect() -> &'static str;
    fn description() -> &'static str;
    fn action(game: &mut Game);
}

#[inline(always)]
pub(crate) fn construct<T: Structure>(ui: &Ui, game: &mut Game)
where
    [(); T::SIZE]:,
{
    // ui.show_demo_window(&mut true);
    // let size = ui.window_size();
    // let win_padding = unsafe { ui.style() }.window_padding[0];
    // let padding = unsafe { ui.style() }.frame_padding[0];
    // let width = (size[0] / 4.0) - 2.0 * padding - 2.0 * win_padding;

    if game.is_unlocked(T::ID) {
        let style = unsafe { ui.style() };
        let mut width = ui.window_size()[0];
        width -= 2.0 * style.window_padding[0];
        width -= 6.0 * style.frame_padding[0];
        width /= 4.0;

        game.actions += 1;
        ui.disabled(!game.afford(&T::cost(game)), || {
            if ui.button_with_size(T::title(), [width, 48.0]) {
                T::action(game);
            }
            if ui.is_item_hovered() {
                ui.tooltip_text("tooltip");
            }

            if game.actions % 4 != 0 {
                ui.same_line();
            }
        })
    }

    // let mut but = Button::new(T::title()).unlocked(game.is_unlocked(T::ID));

    // for cost in T::cost(game) {
    //     but = but.add_cost(cost);
    // }

    // but.build::<T>(ui, game);
}
