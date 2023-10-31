use crate::panel::{default_frame, Panel};
use crate::state::State;
use crate::widget::{CoolCard, InstallButton};
use cool::{Cool, SafeCool, COOL_LIST};
use eframe::Frame;
use egui::{hex_color, Context};

#[derive(Debug)]
pub struct CoolList {
    pub state: State,
    cool_list: Vec<CoolCard>,
}

impl CoolList {
    pub fn new(state: State) -> Self {
        let mut cool_list = COOL_LIST
            .read()
            .unwrap()
            .values()
            .map(|c| CoolCard::new(c.clone(), state.clone()))
            .collect::<Vec<_>>();
        cool_list.sort_by(|a, b| a.cool.read().unwrap().cmp(&b.cool.read().unwrap()));

        Self { cool_list, state }
    }

    fn title(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_height(48.0);
            ui.set_width(ui.available_width());

            let button_width = 200.0;
            let button_height = 58.0;

            ui.label(
                egui::RichText::new("CoolBox")
                    .font(egui::FontId::proportional(36.0))
                    .color(hex_color!("#F5F5F6"))
                    .strong(),
            );
            ui.separator();
            ui.label(
                egui::RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                    .font(egui::FontId::proportional(24.0))
                    .color(hex_color!("#94969C")),
            );
            ui.add_space(ui.available_width() - button_width);
            InstallButton::new(None, Some(button_width), Some(button_height)).show(ui);
        });
    }

    fn cool_list(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // ui.set_width(ui.available_width());

            self.cool_list.iter_mut().for_each(|cool| {
                cool.show(ui);
            });
        });
    }
}

impl Panel for CoolList {
    fn ui(&mut self, ctx: &Context, frame: &mut Frame) {
        egui::CentralPanel::default()
            .frame(default_frame())
            .show(ctx, |ui| {
                self.title(ui);
                ui.add_space(28.0);
                self.cool_list(ui);
            });
    }
}
