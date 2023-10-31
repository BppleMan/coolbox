use std::sync::Once;

use crate::panel::{CoolList, Panel};
use eframe::epaint::FontFamily;
use egui::style::DebugOptions;
use egui::{hex_color, FontData, FontDefinitions, Rounding};
pub use state::State;

pub mod panel;
mod state;
pub mod view;
pub mod widget;

static SET_STYLE: Once = Once::new();

#[no_mangle]
pub fn render(state: &mut State, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    set_style(ctx);
    CoolList::new(state.clone()).ui(ctx, _frame);
}

pub fn set_style(ctx: &egui::Context) {
    SET_STYLE.call_once(|| {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "iconfont".to_string(),
            FontData::from_static(include_bytes!("../../assets/fonts/iconfont.ttf")),
        );
        fonts.font_data.insert(
            "PingFang-SC-Regular".to_string(),
            FontData::from_static(include_bytes!("../../assets/fonts/PingFang-SC-Regular.ttf")),
        );
        fonts.font_data.insert(
            "JetBrainsMono-Regular".to_string(),
            FontData::from_static(include_bytes!(
                "../../assets/fonts/JetBrainsMono-Regular.ttf"
            )),
        );

        let mono = fonts.families.entry(FontFamily::Monospace).or_default();
        mono.insert(0, "JetBrainsMono-Regular".to_string());
        mono.insert(1, "iconfont".to_string());

        let proportional = fonts.families.entry(FontFamily::Proportional).or_default();
        proportional.insert(0, "PingFang-SC-Regular".to_string());

        ctx.set_fonts(fonts);
        let mut style = (*ctx.style()).clone();
        let _debug_options = DebugOptions {
            show_blocking_widget: true,
            show_expand_width: true,
            show_expand_height: true,
            show_resize: true,
            ..Default::default()
        };
        // style.debug = debug_options;
        style.visuals.override_text_color = Some(hex_color!("#FFFFFF"));
        // style.visuals.widgets.noninteractive = egui::style::WidgetVisuals {
        //     bg_fill: hex_color!("#1F242F"),
        //     bg_stroke: egui::Stroke::new(1.0, hex_color!("#1F242F")),
        //     rounding: Rounding::same(8.0),
        //     ..style.visuals.widgets.noninteractive
        // };
        // style.visuals.widgets.inactive = egui::style::WidgetVisuals {
        //     bg_fill: egui::Color32::TRANSPARENT,
        //     weak_bg_fill: egui::Color32::TRANSPARENT,
        //     bg_stroke: egui::Stroke::new(1.0, hex_color!("#1F242F")),
        //     rounding: Rounding::same(8.0),
        //     ..style.visuals.widgets.inactive
        // };

        style.visuals.widgets.hovered.weak_bg_fill = hex_color!("#333741");
        style.visuals.widgets.hovered.rounding = Rounding::same(12.0);

        style.visuals.widgets.active.rounding = Rounding::same(12.0);

        style.visuals.panel_fill = hex_color!("#141414");
        style.visuals.window_fill = hex_color!("#141414");
        ctx.set_style(style);
    });
}
