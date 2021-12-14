pub enum AnsiColors {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

fn get_code(ac: AnsiColors) -> i32 {
    match ac {
        AnsiColors::Reset => 0,
        AnsiColors::Black => 30,
        AnsiColors::Red => 31,
        AnsiColors::Green => 32,
        AnsiColors::Yellow => 33,
        AnsiColors::Blue => 34,
        AnsiColors::Magenta => 35,
        AnsiColors::Cyan => 36,
        AnsiColors::White => 37,
    }
}

pub fn set_color(clr: AnsiColors) {
    let start = "\x1b[";
    let end = "m";
    print!("{}{}{}", start, get_code(clr), end);
}

pub fn print_in_color(s: &str, clr: AnsiColors) {
    let start = "\x1b[";
    let end = "m";
    print!("{}{}{}{}", start, get_code(clr), end, s);
}
