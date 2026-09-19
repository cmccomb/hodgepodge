//! Color datasets ranging from rainbow mnemonics (ROYGBIV) to CMYK, RGB, and
//! CSS keyword lists. Use `rgb()` for packed RGB values; enum discriminants
//! are identifiers and do not represent colors.
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]
#![allow(clippy::unreadable_literal)]

color_enum! {
    /// ROYGBIV colors, with hex codes as found [here](https://www.webnots.com/vibgyor-rainbow-color-codes/)
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
}

#[cfg(test)]
mod test_roygbiv {
    use crate::ROYGBIV;

    #[test]
    fn rgb_values_match_palette_swatches() {
        assert_eq!(ROYGBIV::Red.rgb(), 0xff0000);
        assert_eq!(ROYGBIV::Green.rgb(), 0x00ff00);
        assert_eq!(ROYGBIV::Violet.rgb(), 0x9400d3);
    }

    #[test]
    fn formatting_uses_lower_hex() {
        assert_eq!(format!("{:x}", ROYGBIV::Blue), "0000ff");
    }

    #[cfg(feature = "strum")]
    #[test]
    fn roygbiv_has_seven_variants() {
        use strum::EnumCount;

        assert_eq!(<ROYGBIV as EnumCount>::COUNT, 7);
    }
}

color_enum! {
    /// CMYK colors
    pub enum CMYK {
        /// Cyan is the first CMYK channel
        Cyan = 0x00ffff,

        /// Magenta is the second CMYK channel
        Magenta = 0xff00ff,

        /// Yellow is the third CMYK channel
        Yellow = 0xffff00,

        /// Black is the fourth CMYK channel (also known as Key)
        Black = 0x000000,
    }
}

#[cfg(test)]
mod test_cmyk {
    use crate::CMYK;

    #[test]
    fn black_is_zero_and_cmy_channels_are_unique() {
        assert_eq!(CMYK::Black.rgb(), 0x000000);
        assert_ne!(CMYK::Cyan.rgb(), CMYK::Magenta.rgb());
        assert_ne!(CMYK::Yellow.rgb(), CMYK::Cyan.rgb());
    }

    #[test]
    fn lower_hex_representation_zero_pads() {
        assert_eq!(format!("{:x}", CMYK::Black), "000000");
    }

    #[cfg(feature = "strum")]
    #[test]
    fn has_four_printing_channels() {
        use strum::EnumCount;

        assert_eq!(<CMYK as EnumCount>::COUNT, 4);
    }
}

color_enum! {
    /// RGB colors
    pub enum RGB {
        /// Red is the first RGB channel
        Red = 0xff0000,

        /// Green is the second RGB channel
        Green = 0x00ff00,

        /// Blue is the third RGB channel
        Blue = 0x0000ff,
    }
}

#[cfg(test)]
mod test_rgb {
    use crate::RGB;

    #[test]
    fn rgb_channels_match_expected_hex_values() {
        assert_eq!(RGB::Red.rgb(), 0xff0000);
        assert_eq!(RGB::Green.rgb(), 0x00ff00);
        assert_eq!(RGB::Blue.rgb(), 0x0000ff);
    }

    #[test]
    fn rgb_hex_formatting_zero_pads() {
        assert_eq!(format!("{:x}", RGB::Blue), "0000ff");
    }

    #[cfg(feature = "strum")]
    #[test]
    fn exactly_three_rgb_channels_exist() {
        use strum::EnumCount;

        assert_eq!(<RGB as EnumCount>::COUNT, 3);
    }
}

color_enum! {
    /// The 148 opaque [CSS named colors](https://www.w3.org/TR/css-color-4/#named-colors).
    ///
    /// Aliases such as `Aqua` and `Cyan` remain distinct variants with equal `rgb()`
    /// values. Hex formatting uses the exact RGB value for every name.
    pub enum CSS {
        AliceBlue = 0xf0f8ff,
        AntiqueWhite = 0xfaebd7,
        Aquamarine = 0x7fffd4,
        Azure = 0xf0ffff,
        Beige = 0xf5f5dc,
        Bisque = 0xffe4c4,
        Black = 0x000000,
        BlanchedAlmond = 0xffebcd,
        Blue = 0x0000ff,
        BlueViolet = 0x8a2be2,
        Brown = 0xa52a2a,
        BurlyWood = 0xdeb887,
        CadetBlue = 0x5f9ea0,
        Chartreuse = 0x7fff00,
        Chocolate = 0xd2691e,
        Coral = 0xff7f50,
        CornflowerBlue = 0x6495ed,
        Cornsilk = 0xfff8dc,
        Crimson = 0xdc143c,
        Cyan = 0x00ffff,
        DarkBlue = 0x00008b,
        DarkCyan = 0x008b8b,
        DarkGoldenRod = 0xb8860b,
        DarkGray = 0xa9a9a9,
        DarkGreen = 0x006400,
        DarkKhaki = 0xbdb76b,
        DarkMagenta = 0x8b008b,
        DarkOliveGreen = 0x556b2f,
        DarkOrange = 0xff8c00,
        DarkOrchid = 0x9932cc,
        DarkRed = 0x8b0000,
        DarkSalmon = 0xe9967a,
        DarkSeaGreen = 0x8fbc8f,
        DarkSlateBlue = 0x483d8b,
        DarkSlateGray = 0x2f4f4f,
        DarkTurquoise = 0x00ced1,
        DarkViolet = 0x9400d3,
        DeepPink = 0xff1493,
        DeepSkyBlue = 0x00bfff,
        DimGray = 0x696969,
        DodgerBlue = 0x1e90ff,
        FireBrick = 0xb22222,
        FloralWhite = 0xfffaf0,
        ForestGreen = 0x228b22,
        Gainsboro = 0xdcdcdc,
        GhostWhite = 0xf8f8ff,
        Gold = 0xffd700,
        GoldenRod = 0xdaa520,
        Gray = 0x808080,
        Green = 0x008000,
        GreenYellow = 0xadff2f,
        HoneyDew = 0xf0fff0,
        HotPink = 0xff69b4,
        IndianRed = 0xcd5c5c,
        Indigo = 0x4b0082,
        Ivory = 0xfffff0,
        Khaki = 0xf0e68c,
        Lavender = 0xe6e6fa,
        LavenderBlush = 0xfff0f5,
        LawnGreen = 0x7cfc00,
        LemonChiffon = 0xfffacd,
        LightBlue = 0xadd8e6,
        LightCoral = 0xf08080,
        LightCyan = 0xe0ffff,
        LightGoldenRodYellow = 0xfafad2,
        LightGray = 0xd3d3d3,
        LightGreen = 0x90ee90,
        LightPink = 0xffb6c1,
        LightSalmon = 0xffa07a,
        LightSeaGreen = 0x20b2aa,
        LightSkyBlue = 0x87cefa,
        LightSlateGray = 0x778899,
        LightSteelBlue = 0xb0c4de,
        LightYellow = 0xffffe0,
        Lime = 0x00ff00,
        LimeGreen = 0x32cd32,
        Linen = 0xfaf0e6,
        Magenta = 0xff00ff,
        Maroon = 0x800000,
        MediumAquaMarine = 0x66cdaa,
        MediumBlue = 0x0000cd,
        MediumOrchid = 0xba55d3,
        MediumPurple = 0x9370db,
        MediumSeaGreen = 0x3cb371,
        MediumSlateBlue = 0x7b68ee,
        MediumSpringGreen = 0x00fa9a,
        MediumTurquoise = 0x48d1cc,
        MediumVioletRed = 0xc71585,
        MidnightBlue = 0x191970,
        MintCream = 0xf5fffa,
        MistyRose = 0xffe4e1,
        Moccasin = 0xffe4b5,
        NavajoWhite = 0xffdead,
        Navy = 0x000080,
        OldLace = 0xfdf5e6,
        Olive = 0x808000,
        OliveDrab = 0x6b8e23,
        Orange = 0xffa500,
        OrangeRed = 0xff4500,
        Orchid = 0xda70d6,
        PaleGoldenRod = 0xeee8aa,
        PaleGreen = 0x98fb98,
        PaleTurquoise = 0xafeeee,
        PaleVioletRed = 0xdb7093,
        PapayaWhip = 0xffefd5,
        PeachPuff = 0xffdab9,
        Peru = 0xcd853f,
        Pink = 0xffc0cb,
        Plum = 0xdda0dd,
        PowderBlue = 0xb0e0e6,
        Purple = 0x800080,
        RebeccaPurple = 0x663399,
        Red = 0xff0000,
        RosyBrown = 0xbc8f8f,
        RoyalBlue = 0x4169e1,
        SaddleBrown = 0x8b4513,
        Salmon = 0xfa8072,
        SandyBrown = 0xf4a460,
        SeaGreen = 0x2e8b57,
        SeaShell = 0xfff5ee,
        Sienna = 0xa0522d,
        Silver = 0xc0c0c0,
        SkyBlue = 0x87ceeb,
        SlateBlue = 0x6a5acd,
        SlateGray = 0x708090,
        Snow = 0xfffafa,
        SpringGreen = 0x00ff7f,
        SteelBlue = 0x4682b4,
        Tan = 0xd2b48c,
        Teal = 0x008080,
        Thistle = 0xd8bfd8,
        Tomato = 0xff6347,
        Turquoise = 0x40e0d0,
        Violet = 0xee82ee,
        Wheat = 0xf5deb3,
        White = 0xffffff,
        WhiteSmoke = 0xf5f5f5,
        Yellow = 0xffff00,
        YellowGreen = 0x9acd32,

        Aqua = 0x00ffff,

        DarkSlateGrey = 0x2f4f4f,

        Fuchsia = 0xff00ff,

        DimGrey = 0x696969,

        SlateGrey = 0x708090,

        LightSlateGrey = 0x778899,

        Grey = 0x808080,

        DarkGrey = 0xa9a9a9,

        LightGrey = 0xd3d3d3,
    }
}

#[cfg(test)]
mod test_css {
    use crate::CSS;

    #[test]
    fn canonical_css_colors_match_spec() {
        assert_eq!(CSS::RebeccaPurple.rgb(), 0x663399);
        assert_eq!(CSS::White.rgb(), 0xffffff);
        assert_eq!(CSS::Black.rgb(), 0x000000);
    }

    #[test]
    fn aliases_preserve_names_and_share_rgb_values() {
        assert_eq!(CSS::Cyan.rgb(), 0x00ffff);
        assert_eq!(CSS::Aqua.rgb(), 0x00ffff);
        assert_ne!(CSS::Aqua, CSS::Cyan);
        assert_eq!(CSS::Aqua.rgb(), CSS::Cyan.rgb());
        assert_eq!(CSS::Grey.rgb(), CSS::Gray.rgb());
    }

    #[cfg(feature = "strum")]
    #[test]
    fn css_keyword_list_contains_expected_entries() {
        use strum::EnumCount;

        assert_eq!(<CSS as EnumCount>::COUNT, 148);
    }
}
