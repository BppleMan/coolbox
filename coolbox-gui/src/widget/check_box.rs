use eframe::emath::vec2;
use egui::{TextStyle, Widget, WidgetText};

static CHECK_SVG: egui::ImageSource =
    egui::include_image!("../../../assets/images/coolbox-svg/uncheckbox.svg");

const ICON_SIZE: f32 = 24.0;
const ICON_SPACING: f32 = 16.0;
const HEIGHT: f32 = 30.0;

pub struct CheckBox<'a> {
    checked: &'a mut bool,
    label: WidgetText,
    hover: bool,
}

impl<'a> CheckBox<'a> {
    pub fn new(checked: &'a mut bool, label: impl Into<WidgetText>) -> Self {
        Self {
            checked,
            label: label.into(),
            hover: false,
        }
    }

    pub fn interaction(&mut self, ui: &mut egui::Ui) -> (egui::Rect, egui::Response) {
        let total_extra = vec2(ICON_SIZE + ICON_SPACING, HEIGHT);

        let wrap_size = ui.available_size() - total_extra;
        let text = self
            .label
            .clone()
            .into_galley(ui, None, wrap_size.x, TextStyle::Button);

        let desired_size = text.size() + total_extra;
        let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
        if response.hovered() {
            self.hover = true;
        }
        if response.clicked() {
            *self.checked = !*self.checked;
        }
        (rect, response)
    }

    pub fn show_box(
        &mut self,
        rect: egui::Rect,
        response: egui::Response,
        ui: &mut egui::Ui,
    ) -> (egui::Rect, egui::Response) {
        let check_svg = if *self.checked {
            egui::Image::new(egui::include_image!(
                "../../../assets/images/coolbox-svg/checkbox.svg"
            ))
        } else {
            egui::Image::new(egui::include_image!(
                "../../../assets/images/coolbox-svg/uncheckbox.svg"
            ))
        }
        .max_width(ICON_SIZE)
        .max_height(ICON_SIZE)
        .sense(egui::Sense::click())
        .bg_fill(egui::Color32::TRANSPARENT)
        .rounding(egui::Rounding::same(0.0));
        ui.allocate_ui_at_rect(rect, |ui| {
            let min = rect.min + egui::vec2(0.0, (rect.height() - ICON_SIZE) / 2.0);
            let rect = egui::Rect::from_min_size(min, egui::Vec2::splat(ICON_SIZE));
            ui.allocate_space(rect.size());
            check_svg.paint_at(ui, rect);
            if self.hover {
                if *self.checked {
                    egui::Image::new(egui::include_image!(
                        "../../../assets/images/coolbox-svg/checkbox_hover.svg"
                    ))
                } else {
                    egui::Image::new(egui::include_image!(
                        "../../../assets/images/coolbox-svg/uncheckbox_hover.svg"
                    ))
                }
                .max_size(egui::Vec2::splat(ICON_SIZE))
                .paint_at(ui, rect);
            }
        });
        (rect, response)
    }
}

impl<'a> Widget for CheckBox<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.set_height(HEIGHT);
        ui.horizontal(|ui| {
            let (rect, response) = self.interaction(ui);
            let (_rect, response) = self.show_box(rect, response, ui);
            ui.label(self.label);
            response
        })
        .inner
    }
}
