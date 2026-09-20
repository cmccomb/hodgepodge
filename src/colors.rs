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
        Red = 0xff0000 => "Red",

        /// Orange is the second color in ROYGBIV
        Orange = 0xff7f00 => "Orange",

        /// Yellow is the third color in ROYGBIV
        Yellow = 0xffff00 => "Yellow",

        /// Green is the fourth color in ROYGBIV
        Green = 0x00ff00 => "Green",

        /// Blue is the fifth color in ROYGBIV
        Blue = 0x0000ff => "Blue",

        /// Indigo is the sixth color in ROYGBIV
        Indigo = 0x4b0082 => "Indigo",

        /// Violet is the seventh color in ROYGBIV
        Violet = 0x9400d3 => "Violet",
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
        Cyan = 0x00ffff => "Cyan",

        /// Magenta is the second CMYK channel
        Magenta = 0xff00ff => "Magenta",

        /// Yellow is the third CMYK channel
        Yellow = 0xffff00 => "Yellow",

        /// Black is the fourth CMYK channel (also known as Key)
        Black = 0x000000 => "Black",
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
        Red = 0xff0000 => "Red",

        /// Green is the second RGB channel
        Green = 0x00ff00 => "Green",

        /// Blue is the third RGB channel
        Blue = 0x0000ff => "Blue",
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
        AliceBlue = 0xf0f8ff => "Alice Blue",
        AntiqueWhite = 0xfaebd7 => "Antique White",
        Aquamarine = 0x7fffd4 => "Aquamarine",
        Azure = 0xf0ffff => "Azure",
        Beige = 0xf5f5dc => "Beige",
        Bisque = 0xffe4c4 => "Bisque",
        Black = 0x000000 => "Black",
        BlanchedAlmond = 0xffebcd => "Blanched Almond",
        Blue = 0x0000ff => "Blue",
        BlueViolet = 0x8a2be2 => "Blue Violet",
        Brown = 0xa52a2a => "Brown",
        BurlyWood = 0xdeb887 => "Burly Wood",
        CadetBlue = 0x5f9ea0 => "Cadet Blue",
        Chartreuse = 0x7fff00 => "Chartreuse",
        Chocolate = 0xd2691e => "Chocolate",
        Coral = 0xff7f50 => "Coral",
        CornflowerBlue = 0x6495ed => "Cornflower Blue",
        Cornsilk = 0xfff8dc => "Cornsilk",
        Crimson = 0xdc143c => "Crimson",
        Cyan = 0x00ffff => "Cyan",
        DarkBlue = 0x00008b => "Dark Blue",
        DarkCyan = 0x008b8b => "Dark Cyan",
        DarkGoldenRod = 0xb8860b => "Dark Golden Rod",
        DarkGray = 0xa9a9a9 => "Dark Gray",
        DarkGreen = 0x006400 => "Dark Green",
        DarkKhaki = 0xbdb76b => "Dark Khaki",
        DarkMagenta = 0x8b008b => "Dark Magenta",
        DarkOliveGreen = 0x556b2f => "Dark Olive Green",
        DarkOrange = 0xff8c00 => "Dark Orange",
        DarkOrchid = 0x9932cc => "Dark Orchid",
        DarkRed = 0x8b0000 => "Dark Red",
        DarkSalmon = 0xe9967a => "Dark Salmon",
        DarkSeaGreen = 0x8fbc8f => "Dark Sea Green",
        DarkSlateBlue = 0x483d8b => "Dark Slate Blue",
        DarkSlateGray = 0x2f4f4f => "Dark Slate Gray",
        DarkTurquoise = 0x00ced1 => "Dark Turquoise",
        DarkViolet = 0x9400d3 => "Dark Violet",
        DeepPink = 0xff1493 => "Deep Pink",
        DeepSkyBlue = 0x00bfff => "Deep Sky Blue",
        DimGray = 0x696969 => "Dim Gray",
        DodgerBlue = 0x1e90ff => "Dodger Blue",
        FireBrick = 0xb22222 => "Fire Brick",
        FloralWhite = 0xfffaf0 => "Floral White",
        ForestGreen = 0x228b22 => "Forest Green",
        Gainsboro = 0xdcdcdc => "Gainsboro",
        GhostWhite = 0xf8f8ff => "Ghost White",
        Gold = 0xffd700 => "Gold",
        GoldenRod = 0xdaa520 => "Golden Rod",
        Gray = 0x808080 => "Gray",
        Green = 0x008000 => "Green",
        GreenYellow = 0xadff2f => "Green Yellow",
        HoneyDew = 0xf0fff0 => "Honey Dew",
        HotPink = 0xff69b4 => "Hot Pink",
        IndianRed = 0xcd5c5c => "Indian Red",
        Indigo = 0x4b0082 => "Indigo",
        Ivory = 0xfffff0 => "Ivory",
        Khaki = 0xf0e68c => "Khaki",
        Lavender = 0xe6e6fa => "Lavender",
        LavenderBlush = 0xfff0f5 => "Lavender Blush",
        LawnGreen = 0x7cfc00 => "Lawn Green",
        LemonChiffon = 0xfffacd => "Lemon Chiffon",
        LightBlue = 0xadd8e6 => "Light Blue",
        LightCoral = 0xf08080 => "Light Coral",
        LightCyan = 0xe0ffff => "Light Cyan",
        LightGoldenRodYellow = 0xfafad2 => "Light Golden Rod Yellow",
        LightGray = 0xd3d3d3 => "Light Gray",
        LightGreen = 0x90ee90 => "Light Green",
        LightPink = 0xffb6c1 => "Light Pink",
        LightSalmon = 0xffa07a => "Light Salmon",
        LightSeaGreen = 0x20b2aa => "Light Sea Green",
        LightSkyBlue = 0x87cefa => "Light Sky Blue",
        LightSlateGray = 0x778899 => "Light Slate Gray",
        LightSteelBlue = 0xb0c4de => "Light Steel Blue",
        LightYellow = 0xffffe0 => "Light Yellow",
        Lime = 0x00ff00 => "Lime",
        LimeGreen = 0x32cd32 => "Lime Green",
        Linen = 0xfaf0e6 => "Linen",
        Magenta = 0xff00ff => "Magenta",
        Maroon = 0x800000 => "Maroon",
        MediumAquaMarine = 0x66cdaa => "Medium Aqua Marine",
        MediumBlue = 0x0000cd => "Medium Blue",
        MediumOrchid = 0xba55d3 => "Medium Orchid",
        MediumPurple = 0x9370db => "Medium Purple",
        MediumSeaGreen = 0x3cb371 => "Medium Sea Green",
        MediumSlateBlue = 0x7b68ee => "Medium Slate Blue",
        MediumSpringGreen = 0x00fa9a => "Medium Spring Green",
        MediumTurquoise = 0x48d1cc => "Medium Turquoise",
        MediumVioletRed = 0xc71585 => "Medium Violet Red",
        MidnightBlue = 0x191970 => "Midnight Blue",
        MintCream = 0xf5fffa => "Mint Cream",
        MistyRose = 0xffe4e1 => "Misty Rose",
        Moccasin = 0xffe4b5 => "Moccasin",
        NavajoWhite = 0xffdead => "Navajo White",
        Navy = 0x000080 => "Navy",
        OldLace = 0xfdf5e6 => "Old Lace",
        Olive = 0x808000 => "Olive",
        OliveDrab = 0x6b8e23 => "Olive Drab",
        Orange = 0xffa500 => "Orange",
        OrangeRed = 0xff4500 => "Orange Red",
        Orchid = 0xda70d6 => "Orchid",
        PaleGoldenRod = 0xeee8aa => "Pale Golden Rod",
        PaleGreen = 0x98fb98 => "Pale Green",
        PaleTurquoise = 0xafeeee => "Pale Turquoise",
        PaleVioletRed = 0xdb7093 => "Pale Violet Red",
        PapayaWhip = 0xffefd5 => "Papaya Whip",
        PeachPuff = 0xffdab9 => "Peach Puff",
        Peru = 0xcd853f => "Peru",
        Pink = 0xffc0cb => "Pink",
        Plum = 0xdda0dd => "Plum",
        PowderBlue = 0xb0e0e6 => "Powder Blue",
        Purple = 0x800080 => "Purple",
        RebeccaPurple = 0x663399 => "Rebecca Purple",
        Red = 0xff0000 => "Red",
        RosyBrown = 0xbc8f8f => "Rosy Brown",
        RoyalBlue = 0x4169e1 => "Royal Blue",
        SaddleBrown = 0x8b4513 => "Saddle Brown",
        Salmon = 0xfa8072 => "Salmon",
        SandyBrown = 0xf4a460 => "Sandy Brown",
        SeaGreen = 0x2e8b57 => "Sea Green",
        SeaShell = 0xfff5ee => "Sea Shell",
        Sienna = 0xa0522d => "Sienna",
        Silver = 0xc0c0c0 => "Silver",
        SkyBlue = 0x87ceeb => "Sky Blue",
        SlateBlue = 0x6a5acd => "Slate Blue",
        SlateGray = 0x708090 => "Slate Gray",
        Snow = 0xfffafa => "Snow",
        SpringGreen = 0x00ff7f => "Spring Green",
        SteelBlue = 0x4682b4 => "Steel Blue",
        Tan = 0xd2b48c => "Tan",
        Teal = 0x008080 => "Teal",
        Thistle = 0xd8bfd8 => "Thistle",
        Tomato = 0xff6347 => "Tomato",
        Turquoise = 0x40e0d0 => "Turquoise",
        Violet = 0xee82ee => "Violet",
        Wheat = 0xf5deb3 => "Wheat",
        White = 0xffffff => "White",
        WhiteSmoke = 0xf5f5f5 => "White Smoke",
        Yellow = 0xffff00 => "Yellow",
        YellowGreen = 0x9acd32 => "Yellow Green",

        Aqua = 0x00ffff => "Aqua",

        DarkSlateGrey = 0x2f4f4f => "Dark Slate Grey",

        Fuchsia = 0xff00ff => "Fuchsia",

        DimGrey = 0x696969 => "Dim Grey",

        SlateGrey = 0x708090 => "Slate Grey",

        LightSlateGrey = 0x778899 => "Light Slate Grey",

        Grey = 0x808080 => "Grey",

        DarkGrey = 0xa9a9a9 => "Dark Grey",

        LightGrey = 0xd3d3d3 => "Light Grey",
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
