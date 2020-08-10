//! A module for color enums

// Enables use as an iterable and computation of length
use strum_macros::{EnumIter, EnumCount};

/// ROYGBIV colors
#[derive(Debug, EnumIter, EnumCount)]
pub enum ROYGBIV {
    /// Red is the first color in ROYGBIV
    Red,

    /// Orange is the second color in ROYGBIV
    Orange,

    /// Yellow is the third color in ROYGBIV
    Yellow,

    /// Green is the fourth color in ROYGBIV
    Green,

    /// Blue is the fifth color in ROYGBIV
    Blue,

    /// Indigo is the sixth color in ROYGBIV
    Indigo,

    /// Violet is the seventh color in ROYGBIV
    Violet
}

/// CMYK colors
#[derive(Debug, EnumIter, EnumCount)]
pub enum CMYK {
    /// Cyan is the first CMYK channel
    Cyan,

    /// Magneta is the second CMYK channel
    Magenta,

    /// Yellow is the third CMYK channel
    Yellow,

    /// Black is the fourth CMYK channel (also known as Key)
    Black
}

/// RGB colors
#[derive(Debug, EnumIter, EnumCount)]
pub enum RGB{
    /// Red is the first RGB channel
    Red,

    /// Green is the second RGB channel
    Green,

    /// Blue is the third RGB channel
    Blue
}
