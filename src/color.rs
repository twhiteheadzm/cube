use std::convert::TryFrom;
pub struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
impl From<(i32, i32, i32)> for Color {
    fn from((r, g, b): (i32, i32, i32)) -> Color {
        Color {
            red: u8::try_from(r).unwrap(),
            green: u8::try_from(g).unwrap(),
            blue: u8::try_from(b).unwrap(),
        }
    }
}
impl From<i32> for Color {
    fn from(number: i32) -> Color {
        Color::from((
            number / (256 * 256),
            (number % (256 * 256)) / 256,
            number % 256,
        ))
    }
}
impl Into<i32> for Color {
    fn into(self) -> i32 {
        i32::from(self.red) * 256 * 256 + i32::from(self.green) * 256 + i32::from(self.blue)
    }
}
impl Color {
    pub fn darken(self, x: i32) -> Color {
        let x = match x {
            x if x > 255 => 255,
            x if x < 0 => 0,
            _ => u8::try_from(x).unwrap(),
        };
        Color {
            red: self.red.saturating_sub(x),
            green: self.green.saturating_sub(x),
            blue: self.blue.saturating_sub(x),
        }
    }
    pub fn lighten(self, x: i32) -> Color {
        let x = match x {
            x if x > 255 => 255,
            x if x < 0 => 0,
            _ => u8::try_from(x).unwrap(),
        };
        Color {
            red: self.red.saturating_add(x),
            green: self.green.saturating_add(x),
            blue: self.blue.saturating_add(x),
        }
    }
}
