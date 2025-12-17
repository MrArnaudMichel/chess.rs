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

pub fn color_to_string(color: u8) -> &'static str {
    match color {
        WHITE => "White",
        BLACK => "Black",
        _ => "Unknown",
    }
}