use ps2_filetypes::ColorF;
use ps2_filetypes::color::Color;
use regex::Regex;

pub trait HexCode {
    fn is_hexcode(&self) -> bool;
}

impl HexCode for String {
    fn is_hexcode(&self) -> bool {
        Regex::new(r"(?i)^#[0-9a-f]{6}$").unwrap().is_match(self)
    }
}

pub trait HexColors {
    fn to_hex(&self) -> String;

    fn update_from_hex(&mut self, hex_code: String);
}

impl HexColors for Color {
    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    fn update_from_hex(&mut self, hex_code: String) {
        self.r = u8::from_str_radix(&hex_code[1..3],16).unwrap();
        self.g = u8::from_str_radix(&hex_code[3..5],16).unwrap();
        self.b = u8::from_str_radix(&hex_code[5..7],16).unwrap();
    }
}

impl HexColors for ColorF {
    fn to_hex(&self) -> String {
        format!(
            "#{:02X}{:02X}{:02X}",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        )
    }

    fn update_from_hex(&mut self, hex_code: String) {
        if hex_code.is_hexcode() {
            self.r = u8::from_str_radix(&hex_code[1..3],16).unwrap() as f32 / 255.0;
            self.g = u8::from_str_radix(&hex_code[3..5],16).unwrap() as f32 / 255.0;
            self.b = u8::from_str_radix(&hex_code[5..7],16).unwrap() as f32 / 255.0;
        }
    }
}
