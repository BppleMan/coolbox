pub trait View {
    fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context, frame: &mut eframe::Frame);
}
