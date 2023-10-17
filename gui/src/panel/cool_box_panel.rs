use crate::panel::{default_frame, Panel};
use crate::widget::show_install_button;
use eframe::Frame;
use egui::Context;

#[derive(Debug, Default)]
pub struct CoolBoxPanel {}

impl CoolBoxPanel {
    fn title(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_height(58.0);
            ui.set_width(ui.available_width());

            let button_width = 200.0;
            let button_height = 58.0;

            ui.label("CoolBox");
            ui.separator();
            ui.label(env!("CARGO_PKG_VERSION"));
            ui.add_space(ui.available_width() - button_width);

            let (button, install_receiver, progress_sender) =
                show_install_button(ui, Some(button_width), Some(button_height));
        });
    }
}

impl Panel for CoolBoxPanel {
    fn ui(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default()
            .frame(default_frame())
            .show(ctx, |ui| {
                self.title(ui);
            });
    }
}
