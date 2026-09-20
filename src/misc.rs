//! Miscellaneous datasets such as medals, ordinals, and programming trivia.
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

dataset_enum! {
    /// Programming languages better than Rust
    pub enum BetterThanRust {
        /// That's right, there aren't any.
        None = 0 => "None",
    }
}

#[cfg(test)]
mod better_than_rust_tests {
    use super::BetterThanRust;

    #[test]
    fn there_are_no_languages_better_than_rust() {
        assert_eq!(BetterThanRust::None as i32, 0);
    }

    #[cfg(feature = "strum")]
    #[test]
    fn enum_contains_single_variant() {
        use strum::EnumCount;

        assert_eq!(<BetterThanRust as EnumCount>::COUNT, 1);
    }
}

dataset_enum! {
    /// Standard medals
    pub enum Medal {
        /// A [Gold Medal](https://en.wikipedia.org/wiki/Gold_medal) is typical awarded for first place
        Gold = 1 => "Gold",

        /// A [Silver Medal](https://en.wikipedia.org/wiki/Silver_medal) is typical awarded for second place
        Silver = 2 => "Silver",

        /// A [Bronze Medal](https://en.wikipedia.org/wiki/Bronze_medal) is typical awarded for third place
        Bronze = 3 => "Bronze",
    }
}

dataset_enum! {
    /// English ordinal names for 1 through 30.
    #[allow(missing_docs)]
    pub enum Ordinal {
        First = 1 => "First",
        Second = 2 => "Second",
        Third = 3 => "Third",
        Fourth = 4 => "Fourth",
        Fifth = 5 => "Fifth",
        Sixth = 6 => "Sixth",
        Seventh = 7 => "Seventh",
        Eighth = 8 => "Eighth",
        Ninth = 9 => "Ninth",
        Tenth = 10 => "Tenth",
        Eleventh = 11 => "Eleventh",
        Twelfth = 12 => "Twelfth",
        Thirteenth = 13 => "Thirteenth",
        Fourteenth = 14 => "Fourteenth",
        Fifteenth = 15 => "Fifteenth",
        Sixteenth = 16 => "Sixteenth",
        Seventeenth = 17 => "Seventeenth",
        Eighteenth = 18 => "Eighteenth",
        Nineteenth = 19 => "Nineteenth",
        Twentieth = 20 => "Twentieth",
        Twentyfirst = 21 => "Twenty-first",
        Twentysecond = 22 => "Twenty-second",
        Twentythird = 23 => "Twenty-third",
        Twentyfourth = 24 => "Twenty-fourth",
        Twentyfifth = 25 => "Twenty-fifth",
        Twentysixth = 26 => "Twenty-sixth",
        Twentyseventh = 27 => "Twenty-seventh",
        Twentyeighth = 28 => "Twenty-eighth",
        Twentyninth = 29 => "Twenty-ninth",
        Thirtieth = 30 => "Thirtieth",
    }
}

impl Medal {
    /// Returns the conventional podium position (Gold = 1).
    #[must_use]
    pub const fn place(self) -> u8 {
        self as u8
    }
}

impl Ordinal {
    /// Returns the ordinal's integer value, from 1 to 30.
    #[must_use]
    pub const fn number(self) -> u8 {
        self as u8
    }
}

checked_u8_enum!(Medal, place);
checked_u8_enum!(Ordinal, number);
