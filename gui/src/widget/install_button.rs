use coolbox_core::color_eyre::eyre::eyre;
use coolbox_core::progress::InstallProgress;
use coolbox_core::tracing::error;
use crossbeam::channel::{Receiver, Sender};
use egui::hex_color;
use std::ops::DerefMut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

pub struct InstallButton {
    install_sender: Sender<()>,
    pub progress: Arc<RwLock<InstallProgress>>,
    should_stop: Arc<AtomicBool>,
    stop_handle: Receiver<()>,

    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl InstallButton {
    pub fn new(
        install_sender: Sender<()>,
        installed_receiver: Receiver<InstallProgress>,
        width: Option<f32>,
        height: Option<f32>,
    ) -> Self {
        let progress = Arc::new(RwLock::new(InstallProgress::NOT_STARTED));
        let cloned_progress = progress.clone();
        let should_stop = Arc::new(AtomicBool::new(false));
        let cloned_should_stop = should_stop.clone();
        let (stop_sender, stop_receiver) = crossbeam::channel::bounded(1);
        rayon::spawn(move || {
            while !cloned_should_stop.load(Ordering::Relaxed) {
                match installed_receiver.recv() {
                    Ok(mut progress) => {
                        std::mem::swap(cloned_progress.write().unwrap().deref_mut(), &mut progress)
                    }
                    Err(e) => {
                        error!("Error while receiving progress: {}", e);
                        break;
                    }
                }
            }
            stop_sender.send(()).map_err(|e| eyre!(e)).unwrap()
        });
        Self {
            install_sender,
            progress,
            should_stop,
            stop_handle: stop_receiver,

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

impl Drop for InstallButton {
    fn drop(&mut self) {
        self.should_stop.store(true, Ordering::Relaxed);
        self.stop_handle.recv().unwrap();
    }
}

impl InstallButton {
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

pub fn show_install_button(
    ui: &mut egui::Ui,
    width: Option<f32>,
    height: Option<f32>,
) -> (InstallButton, Receiver<()>, Sender<InstallProgress>) {
    let (install_sender, installed_receiver) = crossbeam::channel::bounded(1);
    let (progress_sender, progress_receiver) = crossbeam::channel::unbounded();
    let mut button = InstallButton::new(install_sender, progress_receiver, width, height);
    button.show(ui);
    (button, installed_receiver, progress_sender)
}
