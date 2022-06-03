use crate::{resource::ResourceType, Game};
use imgui::{sys::ImGuiCol_Text, ImColor32, ItemHoveredFlags, Ui};

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

#[inline(always)]
pub(crate) fn construct_impl<T: Structure>(ui: &Ui, game: &mut Game, count: Option<u32>)
where
    [(); T::SIZE]:,
{
    if game.is_unlocked(T::ID) {
        let style = unsafe { ui.style() };
        let mut width = ui.window_size()[0];
        width -= 2.0 * style.window_padding[0];
        width -= 6.0 * style.frame_padding[0];
        width /= 4.0;
        let size = [width, 48.0];

        game.actions += 1;
        let mut p1 = ui.cursor_screen_pos();
        ui.enabled(game.afford(&T::cost(game)), || {
            if ui.button_with_size(T::title(), size) {
                T::action(game);
            }
        });
        if ui.is_item_hovered_with_flags(ItemHoveredFlags::ALLOW_WHEN_DISABLED) {
            T::tooltip(ui, game);
        }

        if let Some(count) = count {
            if count != 0 {
                let text = format!("{count}");
                let text_size = ui.calc_text_size(&text);
                p1[0] += width - text_size[0] - 7.0;
                let p2 = [p1[0] + text_size[0] + 7.0, p1[1] + text_size[1] + 1.0];

                let draw = ui.get_window_draw_list();

                draw.add_rect(p1, p2, ImColor32::from_rgb(40, 40, 40))
                    .filled(true)
                    .rounding(5.0)
                    .round_bot_right(false)
                    .round_top_left(false)
                    .round_top_right(false)
                    .build();

                draw.add_text([p1[0] + 4.0, p1[1]], style.colors[ImGuiCol_Text as usize], text);
            }
        }

        if game.actions % 4 != 0 {
            ui.same_line();
        }
    }
}

#[macro_export]
macro_rules! construct {
    ( $T:ty, $ui:ident, $game:ident ) => {
        crate::structure::construct_impl::<$T>(&$ui, $game, None);
    };
    ( $T:ty, $ui:ident, $game:ident  .evolution. $building:ident ) => {
        crate::structure::construct_impl::<$T>(&$ui, $game, $game.evolution.$building.map(|x| x as u32));
    };
}
