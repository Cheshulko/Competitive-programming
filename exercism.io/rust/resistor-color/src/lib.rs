use enum_iterator::{all, Sequence};
use int_enum::IntEnum;
use std::fmt;

#[repr(u32)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, IntEnum, Sequence)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value() as u32
}

pub fn value_to_color_string(value: u32) -> String {
    format!(
        "{}",
        match ResistorColor::from_int(value) {
            Ok(v) => v.to_string(),
            Err(_) => "value out of range".to_string(),
        }
    )
}

pub fn colors() -> Vec<ResistorColor> {
    let mut all = all::<ResistorColor>().collect::<Vec<_>>();
    all.sort_by(|a, b| a.int_value().cmp(&b.int_value()));
    all
}
