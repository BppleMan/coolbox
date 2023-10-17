#[cfg(feature = "hot-reload")]
use crate::hot_lib::render;
#[cfg(not(feature = "hot-reload"))]
use coolbox_gui::render;
use coolbox_gui::State;
use eframe::Frame;
use egui::Context;

#[derive(Debug, Default)]
pub struct CoolBoxApp {
    pub state: State,
}

impl eframe::App for CoolBoxApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        render(&mut self.state, ctx, frame);
    }
}
