use cool::Cool;
use egui::hex_color;

pub struct InstallButton<'a> {
    pub cool: &'a mut Cool,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl<'a> InstallButton<'a> {
    pub fn new(cool: &'a mut Cool, width: Option<f32>, height: Option<f32>) -> Self {
        Self {
            cool,
            width,
            height,
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }
}

impl<'a> InstallButton<'a> {
    pub fn show(&mut self, ui: &mut egui::Ui) -> egui::InnerResponse<()> {
        ui.scope(|ui| {
            ui.style_mut().visuals.widgets.inactive.rounding = egui::Rounding::same(12.0);
            ui.style_mut().visuals.widgets.inactive.weak_bg_fill = hex_color!("#434343");
            ui.spacing_mut().button_padding = egui::vec2(0.0, 0.0);

            let button = egui::Button::new(
                egui::RichText::new("Install")
                    .font(egui::FontId::new(20.0, egui::FontFamily::Proportional)),
            );
            if let (Some(width), Some(height)) = (self.width, self.height) {
                ui.add_sized(egui::vec2(width, height), button);
            } else {
                ui.add(button);
            }
        })
    }
}

// pub fn show_install_button(
//     ui: &mut egui::Ui,
//     width: Option<f32>,
//     height: Option<f32>,
// ) -> (InstallButton, Receiver<()>, Sender<InstallProgress>) {
//     let (install_sender, installed_receiver) = crossbeam::channel::bounded(1);
//     let (progress_sender, progress_receiver) = crossbeam::channel::unbounded();
//     let mut button = InstallButton::new(install_sender, progress_receiver, width, height);
//     button.show(ui);
//     (button, installed_receiver, progress_sender)
// }
