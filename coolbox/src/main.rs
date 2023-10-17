use crate::app::CoolBoxApp;
use coolbox_core::color_eyre::eyre::eyre;
use coolbox_core::{init_backtrace, CBResult};

mod app;

#[cfg(feature = "hot-reload")]
#[hot_lib_reloader::hot_module(
dylib = "coolbox_gui",
lib_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../target/debug")
)]
mod hot_lib {
    use coolbox_gui::*;

    hot_functions_from_file!("gui/src/lib.rs");

    #[lib_change_subscription]
    pub fn subscribe() -> hot_lib_reloader::LibReloadObserver {}
}

fn main() -> CBResult<()> {
    init_backtrace();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1024.0, 768.0)),
        min_window_size: Some(egui::vec2(640.0, 480.0)),
        // decorated: true,
        // transparent: true,
        resizable: true,
        ..Default::default()
    };

    eframe::run_native(
        "Livelink",
        options,
        Box::new(|_cc| {
            #[cfg(feature = "hot-reload")]
            {
                let ctx = _cc.egui_ctx.clone();
                std::thread::spawn(move || loop {
                    hot_lib::subscribe().wait_for_reload();
                    ctx.request_repaint();
                });
            }
            Box::new(CoolBoxApp::default())
        }),
    )
    .map_err(|e| eyre!(e.to_string()))?;
    Ok(())
}
