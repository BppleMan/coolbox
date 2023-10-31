use crate::panel::default_frame;
use crate::state::State;
use crate::widget::{CheckBox, InstallButton};
use cool::SafeCool;
use egui::Key::P;
use egui::{hex_color, Widget};
use std::sync::atomic::Ordering;

#[derive(Debug)]
pub struct CoolCard {
    pub cool: SafeCool,
    pub state: State,
}

impl CoolCard {
    pub fn new(cool: SafeCool, state: State) -> Self {
        Self { cool, state }
    }

    pub fn show_cool_info(&mut self, ui: &mut egui::Ui) {
        let mut checked = self.state.checked.load(Ordering::Relaxed);
        let response = CheckBox::new(&mut checked, self.cool.read().unwrap().name.clone()).ui(ui);
        if response.clicked() {
            self.state.checked.store(checked, Ordering::Relaxed);
        }
    }
}

impl CoolCard {
    pub fn show(&mut self, ui: &mut egui::Ui) -> egui::InnerResponse<()> {
        ui.scope(|ui| {
            // ui.set_width(ui.available_width());

            // ui.style_mut().visuals.widgets.inactive.rounding = egui::Rounding::same(12.0);
            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = hex_color!("#1F242F");
            ui.style_mut().visuals.widgets.inactive.bg_stroke =
                egui::Stroke::new(1.0, hex_color!("#333741"));

            ui.spacing_mut().button_padding = egui::vec2(0.0, 0.0);
            ui.spacing_mut().window_margin = egui::Margin::same(16.0);

            self.show_cool_info(ui);
            /*
            let mut group = default_frame();
            group.rounding = egui::Rounding::same(12.0);
            group.fill = hex_color!("#161B26");
            group.stroke = egui::Stroke::new(1.0, hex_color!("#333741"));
            // group.inner_margin = egui::Margin::same(16.0);
            // group.outer_margin.bottom = 12.0;
            group.show(ui, |ui| {
                self.show_cool_info(ui);
                // ui.spacing_mut().item_spacing = egui::vec2(0.0, 0.0);
                // ui.vertical(|ui| {
                // ui.horizontal(|ui| {
                //     let button_width = 96.0;
                //     let button_height = 36.0;
                //     ui.checkbox(
                //         &mut self.selected,
                //         egui::RichText::new(&self.cool.read().unwrap().name)
                //             .font(egui::FontId::proportional(20.0)),
                //     );
                //     ui.add_space(ui.available_width() - button_width);
                //     InstallButton::new(
                //         Some(self.cool.clone()),
                //         Some(button_width),
                //         Some(button_height),
                //     )
                //     .show(ui);
                // });
                // self.check_box(ui);
                // ui.label(
                //     egui::RichText::new(&self.cool.read().unwrap().description)
                //         .font(egui::FontId::proportional(16.0))
                //         .color(hex_color!("#F5F5F666")),
                // )
                // });
            });
             */
        })
    }
}
