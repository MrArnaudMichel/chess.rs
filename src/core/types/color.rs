pub const WHITE: u8 = 0;
pub const BLACK: u8 = 1;

pub fn invert_color(color: &mut u8) -> u8 {
    *color = match *color {
        WHITE => BLACK,
        BLACK => WHITE,
        _ => *color,
    };
    *color
}