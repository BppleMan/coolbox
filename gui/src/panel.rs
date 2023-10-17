mod cool_box_panel;

pub use cool_box_panel::*;

pub trait Panel {
    fn ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}

pub(crate) fn default_frame() -> egui::Frame {
    egui::Frame {
        inner_margin: egui::Margin::same(40.0),
        outer_margin: egui::Margin::same(0.0),
        rounding: Default::default(),
        shadow: Default::default(),
        fill: egui::hex_color!("#141414"),
        stroke: Default::default(),
    }
}
