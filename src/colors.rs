//! Color datasets ranging from rainbow mnemonics (ROYGBIV) to CMYK, RGB, and
//! curated CSS keyword lists. All enums are C-like, so their discriminants are
//! the RGB hex codes.
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::unreadable_literal)]

// Enables the optional iterator and variant-count derives.
#[cfg(feature = "enum-count")]
use strum_macros::EnumCount;
#[cfg(feature = "enum-iter")]
use strum_macros::EnumIter;

// For deriving lowerhex
use std::fmt;

/// ROYGBIV colors, with hex codes as found [here](https://www.webnots.com/vibgyor-rainbow-color-codes/)
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "enum-iter", derive(EnumIter))]
#[cfg_attr(feature = "enum-count", derive(EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ROYGBIV {
    /// Red is the first color in ROYGBIV
    Red = 0xff0000,

    /// Orange is the second color in ROYGBIV
    Orange = 0xff7f00,

    /// Yellow is the third color in ROYGBIV
    Yellow = 0xffff00,

    /// Green is the fourth color in ROYGBIV
    Green = 0x00ff00,

    /// Blue is the fifth color in ROYGBIV
    Blue = 0x0000ff,

    /// Indigo is the sixth color in ROYGBIV
    Indigo = 0x4b0082,

    /// Violet is the seventh color in ROYGBIV
    Violet = 0x9400d3,
}

impl fmt::LowerHex for ROYGBIV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = *self as i32;
        write!(f, "{x:06x}")
    }
}

#[cfg(test)]
mod roygbiv_tests {
    use super::ROYGBIV;

    #[test]
    fn discriminants_match_rgb_hex_values() {
        assert_eq!(ROYGBIV::Red as i32, 0xff0000);
        assert_eq!(ROYGBIV::Green as i32, 0x00ff00);
        assert_eq!(ROYGBIV::Violet as i32, 0x9400d3);
    }

    #[test]
    fn formatting_uses_lower_hex() {
        assert_eq!(format!("{:x}", ROYGBIV::Blue), "0000ff");
    }

    #[cfg(feature = "enum-count")]
    #[test]
    fn roygbiv_has_seven_variants() {
        use strum::EnumCount;

        assert_eq!(<ROYGBIV as EnumCount>::COUNT, 7);
    }
}

/// CMYK colors
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "enum-iter", derive(EnumIter))]
#[cfg_attr(feature = "enum-count", derive(EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CMYK {
    /// Cyan is the first CMYK channel
    Cyan = 0x00ffff,

    /// Magneta is the second CMYK channel
    Magenta = 0xff00ff,

    /// Yellow is the third CMYK channel
    Yellow = 0xffff00,

    /// Black is the fourth CMYK channel (also known as Key)
    Black = 0x000000,
}

impl fmt::LowerHex for CMYK {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = *self as i32;
        write!(f, "{x:06x}")
    }
}

#[cfg(test)]
mod cmyk_tests {
    use super::CMYK;

    #[test]
    fn black_is_zero_and_cmy_channels_are_unique() {
        assert_eq!(CMYK::Black as i32, 0x000000);
        assert_ne!(CMYK::Cyan as i32, CMYK::Magenta as i32);
        assert_ne!(CMYK::Yellow as i32, CMYK::Cyan as i32);
    }

    #[test]
    fn lower_hex_representation_zero_pads() {
        assert_eq!(format!("{:x}", CMYK::Black), "000000");
    }

    #[cfg(feature = "enum-count")]
    #[test]
    fn has_four_printing_channels() {
        use strum::EnumCount;

        assert_eq!(<CMYK as EnumCount>::COUNT, 4);
    }
}

/// RGB colors
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "enum-iter", derive(EnumIter))]
#[cfg_attr(feature = "enum-count", derive(EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RGB {
    /// Red is the first RGB channel
    Red = 0xff0000,

    /// Green is the second RGB channel
    Green = 0x00ff00,

    /// Blue is the third RGB channel
    Blue = 0x0000ff,
}

impl fmt::LowerHex for RGB {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = *self as i32;
        write!(f, "{x:06x}")
    }
}

#[cfg(test)]
mod rgb_tests {
    use super::RGB;

    #[test]
    fn rgb_channels_match_expected_hex_values() {
        assert_eq!(RGB::Red as i32, 0xff0000);
        assert_eq!(RGB::Green as i32, 0x00ff00);
        assert_eq!(RGB::Blue as i32, 0x0000ff);
    }

    #[test]
    fn rgb_hex_formatting_zero_pads() {
        assert_eq!(format!("{:x}", RGB::Blue), "0000ff");
    }

    #[cfg(feature = "enum-count")]
    #[test]
    fn exactly_three_rgb_channels_exist() {
        use strum::EnumCount;

        assert_eq!(<RGB as EnumCount>::COUNT, 3);
    }
}

/// Color names available in CSS.
///
/// Fuschia, Aqua, and colors that use the 'gr***e***y' instead of 'gr***a***y'
/// are included as variants but only evaluate to an approximately correct hex code (since they
/// share a hex code with other color names). Specifically, the hex code for these variants is
/// incremented by one in the earliest component (i.e., #00FFFF becomes #01FFFF).
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "enum-iter", derive(EnumIter))]
#[cfg_attr(feature = "enum-count", derive(EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(missing_docs)]
pub enum CSS {
    AliceBlue = 0xF0F8FF,
    AntiqueWhite = 0xFAEBD7,
    Aquamarine = 0x7FFFD4,
    Azure = 0xF0FFFF,
    Beige = 0xF5F5DC,
    Bisque = 0xFFE4C4,
    Black = 0x000000,
    BlanchedAlmond = 0xFFEBCD,
    Blue = 0x0000FF,
    BlueViolet = 0x8A2BE2,
    Brown = 0xA52A2A,
    BurlyWood = 0xDEB887,
    CadetBlue = 0x5F9EA0,
    Chartreuse = 0x7FFF00,
    Chocolate = 0xD2691E,
    Coral = 0xFF7F50,
    CornflowerBlue = 0x6495ED,
    Cornsilk = 0xFFF8DC,
    Crimson = 0xDC143C,
    Cyan = 0x00FFFF,
    DarkBlue = 0x00008B,
    DarkCyan = 0x008B8B,
    DarkGoldenRod = 0xB8860B,
    DarkGray = 0xA9A9A9,
    DarkGreen = 0x006400,
    DarkKhaki = 0xBDB76B,
    DarkMagenta = 0x8B008B,
    DarkOliveGreen = 0x556B2F,
    DarkOrange = 0xFF8C00,
    DarkOrchid = 0x9932CC,
    DarkRed = 0x8B0000,
    DarkSalmon = 0xE9967A,
    DarkSeaGreen = 0x8FBC8F,
    DarkSlateBlue = 0x483D8B,
    DarkSlateGray = 0x2F4F4F,
    DarkTurquoise = 0x00CED1,
    DarkViolet = 0x9400D3,
    DeepPink = 0xFF1493,
    DeepSkyBlue = 0x00BFFF,
    DimGray = 0x696969,
    DodgerBlue = 0x1E90FF,
    FireBrick = 0xB22222,
    FloralWhite = 0xFFFAF0,
    ForestGreen = 0x228B22,
    Gainsboro = 0xDCDCDC,
    GhostWhite = 0xF8F8FF,
    Gold = 0xFFD700,
    GoldenRod = 0xDAA520,
    Gray = 0x808080,
    Green = 0x008000,
    GreenYellow = 0xADFF2F,
    HoneyDew = 0xF0FFF0,
    HotPink = 0xFF69B4,
    IndianRed = 0xCD5C5C,
    Indigo = 0x4B0082,
    Ivory = 0xFFFFF0,
    Khaki = 0xF0E68C,
    Lavender = 0xE6E6FA,
    LavenderBlush = 0xFFF0F5,
    LawnGreen = 0x7CFC00,
    LemonChiffon = 0xFFFACD,
    LightBlue = 0xADD8E6,
    LightCoral = 0xF08080,
    LightCyan = 0xE0FFFF,
    LightGoldenRodYellow = 0xFAFAD2,
    LightGray = 0xD3D3D3,
    LightGreen = 0x90EE90,
    LightPink = 0xFFB6C1,
    LightSalmon = 0xFFA07A,
    LightSeaGreen = 0x20B2AA,
    LightSkyBlue = 0x87CEFA,
    LightSlateGray = 0x778899,
    LightSteelBlue = 0xB0C4DE,
    LightYellow = 0xFFFFE0,
    Lime = 0x00FF00,
    LimeGreen = 0x32CD32,
    Linen = 0xFAF0E6,
    Magenta = 0xFF00FF,
    Maroon = 0x800000,
    MediumAquaMarine = 0x66CDAA,
    MediumBlue = 0x0000CD,
    MediumOrchid = 0xBA55D3,
    MediumPurple = 0x9370DB,
    MediumSeaGreen = 0x3CB371,
    MediumSlateBlue = 0x7B68EE,
    MediumSpringGreen = 0x00FA9A,
    MediumTurquoise = 0x48D1CC,
    MediumVioletRed = 0xC71585,
    MidnightBlue = 0x191970,
    MintCream = 0xF5FFFA,
    MistyRose = 0xFFE4E1,
    Moccasin = 0xFFE4B5,
    NavajoWhite = 0xFFDEAD,
    Navy = 0x000080,
    OldLace = 0xFDF5E6,
    Olive = 0x808000,
    OliveDrab = 0x6B8E23,
    Orange = 0xFFA500,
    OrangeRed = 0xFF4500,
    Orchid = 0xDA70D6,
    PaleGoldenRod = 0xEEE8AA,
    PaleGreen = 0x98FB98,
    PaleTurquoise = 0xAFEEEE,
    PaleVioletRed = 0xDB7093,
    PapayaWhip = 0xFFEFD5,
    PeachPuff = 0xFFDAB9,
    Peru = 0xCD853F,
    Pink = 0xFFC0CB,
    Plum = 0xDDA0DD,
    PowderBlue = 0xB0E0E6,
    Purple = 0x800080,
    RebeccaPurple = 0x663399,
    Red = 0xFF0000,
    RosyBrown = 0xBC8F8F,
    RoyalBlue = 0x4169E1,
    SaddleBrown = 0x8B4513,
    Salmon = 0xFA8072,
    SandyBrown = 0xF4A460,
    SeaGreen = 0x2E8B57,
    SeaShell = 0xFFF5EE,
    Sienna = 0xA0522D,
    Silver = 0xC0C0C0,
    SkyBlue = 0x87CEEB,
    SlateBlue = 0x6A5ACD,
    SlateGray = 0x708090,
    Snow = 0xFFFAFA,
    SpringGreen = 0x00FF7F,
    SteelBlue = 0x4682B4,
    Tan = 0xD2B48C,
    Teal = 0x008080,
    Thistle = 0xD8BFD8,
    Tomato = 0xFF6347,
    Turquoise = 0x40E0D0,
    Violet = 0xEE82EE,
    Wheat = 0xF5DEB3,
    White = 0xFFFFFF,
    WhiteSmoke = 0xF5F5F5,
    Yellow = 0xFFFF00,
    YellowGreen = 0x9ACD32,

    /// Note that the associated hex value is not exact. For an exact value, use `Cyan`.
    Aqua = 0x0100ff,

    /// Note that the associated hex value is not exact. For an exact value, use `DarkSlateGray`.
    DarkSlateGrey = 0x304f4f,

    /// Note that the associated hex value is not exact. For an exact value, use `Magenta`.
    Fuschia = 0xff01ff,

    /// Note that the associated hex value is not exact. For an exact value, use `DimGray`.
    DimGrey = 0x706969,

    /// Note that the associated hex value is not exact. For an exact value, use `SlateGray`.
    SlateGrey = 0x718090,

    /// Note that the associated hex value is not exact. For an exact value, use `LightSlateGray`.
    LightSlateGrey = 0x788899,

    /// Note that the associated hex value is not exact. For an exact value, use `Gray`.
    Grey = 0x818080,

    /// Note that the associated hex value is not exact. For an exact value, use `DarkGray`.
    DarkGrey = 0xAAA9A9,

    /// Note that the associated hex value is not exact. For an exact value, use `LightGray`.
    LightGrey = 0xd4d3d3,
}

#[cfg(test)]
mod css_tests {
    use super::CSS;

    #[test]
    fn canonical_css_colors_match_spec() {
        assert_eq!(CSS::RebeccaPurple as i32, 0x663399);
        assert_eq!(CSS::White as i32, 0xffffff);
        assert_eq!(CSS::Black as i32, 0x000000);
    }

    #[test]
    fn approximate_aliases_are_distinct_from_canonical_values() {
        assert_eq!(CSS::Cyan as i32, 0x00ffff);
        assert_eq!(CSS::Aqua as i32, 0x0100ff);
        assert_ne!(CSS::Aqua as i32, CSS::Cyan as i32);
        assert_ne!(CSS::Grey as i32, CSS::Gray as i32);
    }

    #[cfg(feature = "enum-count")]
    #[test]
    fn css_keyword_list_contains_expected_entries() {
        use strum::EnumCount;

        assert_eq!(<CSS as EnumCount>::COUNT, 148);
    }
}

impl fmt::LowerHex for CSS {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x = *self as i32;
        write!(f, "{x:06x}")
    }
}
