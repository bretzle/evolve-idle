use crate::{
    game::{Cost, GameData},
    structure::Structure,
};
use egui::{Layout, Rect, Sense, TextStyle, Ui, Vec2, WidgetInfo, WidgetText, WidgetType};

pub struct Button {
    label: &'static str,
    unlocked: bool,
    cost: Vec<Cost>,
}

impl Button {
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

    pub fn add_cost(mut self, cost: Cost) -> Self {
        self.cost.push(cost);
        self
    }

    pub fn build<T: Structure>(self, ui: &mut Ui, game: &mut GameData) {
        ui.set_enabled(game.afford(&self.cost));
        if self.unlocked {
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
                T::action(game);
            }

            // if response.hovered() {
            //     ui.child_ui(
            //         Rect {
            //             min: [rect.min.x, 0.0].into(),
            //             max: [rect.min.x + desired_size.x * 1.5, desired_size.y * 1.5].into(),
            //         },
            //         Layout::left_to_right(),
            //     )
            //     .vertical_centered(|ui| {
            //         ui.label(T::description());
            //         ui.separator();
            //         ui.label("COST");
            //         ui.separator();
            //         ui.label(T::effect());
            //     });
            // }

            response.on_hover_ui(|ui| {
                ui.label(T::effect());
                // let desired_size = Vec2 {
                //     x: desired_size.x * 1.5,
                //     y: 40.0,
                // };
                // dbg!(desired_size);
                // let description = WidgetText::from(T::description()).into_galley(
                //     ui,
                //     None,
                //     wrap_width,
                //     TextStyle::Body,
                // );

                // let (rect, response) = ui.allocate_at_least(desired_size, Sense::click());

                // let visuals = ui.style().interact(&response);
                // let description_pos = ui
                //     .layout()
                //     .align_size_within_rect(description.size(), rect.shrink2(button_padding))
                //     .min;
                // // // description
                // description.paint_with_visuals(ui.painter(), description_pos, visuals);

                // // // cost

                // // // effect
            });
        }
    }
}
