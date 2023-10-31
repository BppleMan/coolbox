use egui::Widget;

static CHECK_SVG: egui::ImageSource =
    egui::include_image!("../../../assets/images/coolbox-svg/uncheckbox.svg");

pub struct CheckBox<'a> {
    checked: &'a mut bool,
    label: String,
    width: f32,
    hover: bool,
}

impl<'a> CheckBox<'a> {
    pub fn new(checked: &'a mut bool, label: String) -> Self {
        Self {
            checked,
            label,
            width: 24.0,
            hover: false,
        }
    }

    pub fn show_box(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let check_svg = match self.checked {
            true => egui::Image::new(egui::include_image!(
                "../../../assets/images/coolbox-svg/checkbox.svg"
            )),
            false => egui::Image::new(egui::include_image!(
                "../../../assets/images/coolbox-svg/uncheckbox.svg"
            )),
        }
        .max_width(self.width)
        .max_height(self.width)
        .sense(egui::Sense::click())
        .bg_fill(egui::Color32::TRANSPARENT)
        .rounding(egui::Rounding::same(0.0));
        let response = ui.add_sized(egui::Vec2::splat(self.width), check_svg);
        let rect = response.rect;
        if response.hovered() {
            self.hover = true;
        }
        if self.hover {
            egui::Image::new(egui::include_image!(
                "../../../assets/images/coolbox-svg/checkbox_hover.svg"
            ))
            .max_size(egui::Vec2::splat(self.width))
            .paint_at(ui, rect);
        }
        if response.clicked() {
            *self.checked = !*self.checked;
        }
        response
    }
}

impl<'a> Widget for CheckBox<'a> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let inner = ui.horizontal(|ui| {
            let text = egui::RichText::new(&self.label)
                .font(egui::FontId::proportional(20.0))
                .color(egui::hex_color!("#F5F5F6"))
                .strong();
            let response = self.show_box(ui);
            ui.label(
                egui::RichText::new(&self.label)
                    .font(egui::FontId::proportional(20.0))
                    .color(egui::hex_color!("#F5F5F6"))
                    .strong(),
            );
            response
        });
        // let rect = inner.response.rect;
        ui.checkbox(self.checked, &self.label);
        // if inner.response.hovered() {
        //     println!("hovered");
        //     self.hover = true;
        // }
        inner.inner
    }
}
