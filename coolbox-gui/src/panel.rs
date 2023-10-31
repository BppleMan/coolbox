mod cool_list;

pub use cool_list::*;

pub trait Panel {
    fn ui(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
}

pub(crate) fn default_frame() -> egui::Frame {
    egui::Frame {
        inner_margin: egui::Margin::same(40.0),
        outer_margin: egui::Margin::same(0.0),
        rounding: Default::default(),
        shadow: Default::default(),
        fill: egui::hex_color!("#0C111D"),
        stroke: Default::default(),
    }
}
