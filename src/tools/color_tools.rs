use image::Rgb;

pub type ColorType = Rgb<u8>;
#[allow(unused)]
#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    DarkRed,
    LightRed,
    Green,
    DarkGreen,
    LightGreen,
    Blue,
    DarkBlue,
    LightBlue,
    White,
    DarkWhite,
    LightWhite,
    Black,
    DarkBlack,
    LightBlack,
    Yellow,
    DarkYellow,
    LightYellow,
    Cyan,
    DarkCyan,
    LightCyan,
    Magenta,
    DarkMagenta,
    LightMagenta,
    Gray,
    DarkGray,
    LightGray,
    Orange,
    DarkOrange,
    LightOrange,
    Purple,
    DarkPurple,
    LightPurple,
    Brown,
    DarkBrown,
    LightBrown,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> ColorType {
        Rgb([r, g, b])
    }

    pub fn rgb(&self) -> ColorType {
        match self {
            Color::Red => Rgb([255, 0, 0]),
            Color::DarkRed => Rgb([139, 0, 0]),
            Color::LightRed => Rgb([255, 102, 102]),
            Color::Green => Rgb([0, 255, 0]),
            Color::DarkGreen => Rgb([0, 100, 0]),
            Color::LightGreen => Rgb([144, 238, 144]),
            Color::Blue => Rgb([0, 0, 255]),
            Color::DarkBlue => Rgb([0, 0, 139]),
            Color::LightBlue => Rgb([173, 216, 230]),
            Color::White => Rgb([255, 255, 255]),
            Color::DarkWhite => Rgb([211, 211, 211]),
            Color::LightWhite => Rgb([255, 255, 255]),
            Color::Black => Rgb([0, 0, 0]),
            Color::DarkBlack => Rgb([0, 0, 0]),
            Color::LightBlack => Rgb([64, 64, 64]),
            Color::Yellow => Rgb([255, 255, 0]),
            Color::DarkYellow => Rgb([139, 139, 0]),
            Color::LightYellow => Rgb([255, 255, 153]),
            Color::Cyan => Rgb([0, 255, 255]),
            Color::DarkCyan => Rgb([0, 139, 139]),
            Color::LightCyan => Rgb([224, 255, 255]),
            Color::Magenta => Rgb([255, 0, 255]),
            Color::DarkMagenta => Rgb([139, 0, 139]),
            Color::LightMagenta => Rgb([255, 153, 255]),
            Color::Gray => Rgb([128, 128, 128]),
            Color::DarkGray => Rgb([105, 105, 105]),
            Color::LightGray => Rgb([211, 211, 211]),
            Color::Orange => Rgb([255, 165, 0]),
            Color::DarkOrange => Rgb([255, 140, 0]),
            Color::LightOrange => Rgb([255, 200, 0]),
            Color::Purple => Rgb([128, 0, 128]),
            Color::DarkPurple => Rgb([75, 0, 130]),
            Color::LightPurple => Rgb([216, 191, 216]),
            Color::Brown => Rgb([165, 42, 42]),
            Color::DarkBrown => Rgb([92, 51, 23]),
            Color::LightBrown => Rgb([210, 180, 140]),
        }
    }
}
