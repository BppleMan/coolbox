use iced::Color;

pub trait RGB {
    fn to_rgba(&self) -> Color;
}

impl RGB for (&'static str, f32) {
    fn to_rgba(&self) -> Color {
        let hex = self.0;
        let hex = hex.trim_start_matches('#');
        let hex = hex.trim_start_matches("0x");
        let hex = hex.trim_start_matches("0X");
        let hex = hex.trim_start_matches("0b");
        let hex = hex.trim_start_matches("0B");
        let hex = hex.trim_start_matches("0o");
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        let a = self.1;
        Color::from_rgba8(r, g, b, a)
    }
}

impl RGB for u32 {
    fn to_rgba(&self) -> Color {
        let hex = self;
        let r = (hex & 0xff000000) >> 24;
        let g = (hex & 0x00ff0000) >> 16;
        let b = (hex & 0x0000ff00) >> 8;
        let a = (hex & 0x000000ff);
        Color::from_rgba8(r as u8, g as u8, b as u8, a as f32)
    }
}
