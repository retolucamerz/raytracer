use std::ops;
use wasm_bindgen::prelude::*;

use super::structs::*;

#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Color {
    pub r: Float,
    pub g: Float,
    pub b: Float,
    pub a: Float,
}

impl Color {
    pub fn scale(self, s: Float) -> Color {
        Color {
            r: s * self.r,
            g: s * self.g,
            b: s * self.b,
            a: s * self.a,
        }
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
            a: self.a + other.a,
        }
    }
}
impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl ops::Sub<Color> for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        Color {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b,
            a: self.a - other.a,
        }
    }
}

pub const COLOR_ZERO: Color = Color {
    r: 0.,
    g: 0.,
    b: 0.,
    a: 0.,
};
pub const COLOR_BLACK: Color = Color {
    r: 0.,
    g: 0.,
    b: 0.,
    a: 1.,
};
pub const COLOR_WHITE: Color = Color {
    r: 1.,
    g: 1.,
    b: 1.,
    a: 1.,
};
pub const COLOR_ORANGE: Color = Color {
    r: 0.99607843,
    g: 0.64705882,
    b: 0.,
    a: 1.,
};
pub const COLOR_RED: Color = Color {
    r: 1.,
    g: 0.,
    b: 0.,
    a: 1.,
};
pub const COLOR_GREEN: Color = Color {
    r: 0.,
    g: 1.,
    b: 0.,
    a: 1.,
};
pub const COLOR_BLUE: Color = Color {
    r: 0.,
    g: 0.,
    b: 1.,
    a: 1.,
};
