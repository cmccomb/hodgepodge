//! A module for color enums

use strum_macros::EnumIter;

/// ROYGBIV colors
#[derive(Debug, EnumIter)]
pub enum ROYGBIV {
    Red, Orange, Yellow, Green, Blue, Indigo, Violet
}

/// CMYK colors
#[derive(Debug, EnumIter)]
pub enum CMYK {
    Cyan, Magenta, Yellow, Black
}

/// RGB colors
#[derive(Debug, EnumIter)]
pub enum RGB{
    Red, Green, Blue
}
