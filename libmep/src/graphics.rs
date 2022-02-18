#[derive(Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

pub struct Style {
    current_color: Color,
}

impl Style {
    pub fn new() -> Self {
        return Self {
            current_color: Color {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 0,
            },
        };
    }
}
