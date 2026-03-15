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

    fn update_from_hex(&self, hex_code: String);

    fn is_hexcode(text: String) -> bool {
        Regex::new(r"#[0-9a-f]{6}").unwrap().is_match(&*text)
    }
}

impl HexColors for Color {
    fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    fn update_from_hex(&self, hex_code: String) {
        todo!()
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

    fn update_from_hex(&self, hex_code: String) {
        todo!()
    }
}
