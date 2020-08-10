//! A module for time enums

// Enables use as an iterable and computation of length
use strum_macros::{EnumIter, EnumCount};

/// The days of the week
#[derive(Debug, EnumIter, EnumCount, Copy, Clone)]
pub enum Day {
    /// [Monday](https://en.wikipedia.org/wiki/) is the first day of the week
    Monday = 1,

    /// [Tuesday](https://en.wikipedia.org/wiki/Tuesday) is the second day of the week
    Tuesday = 2,

    /// [Wednesday](https://en.wikipedia.org/wiki/Wednesday) is the third day of the week
    Wednesday = 3,

    /// [Thursday](https://en.wikipedia.org/wiki/Thursday) is the fourth day of the week
    Thursday = 4,

    /// [Friday](https://en.wikipedia.org/wiki/Friday) is the fifth day of the week
    Friday = 5,

    /// [Saturday](https://en.wikipedia.org/wiki/Saturday) is the sixth day of the week
    Saturday = 6,

    /// [Sunday](https://en.wikipedia.org/wiki/Sunday) is the seventh day of the week
    Sunday = 7,
}

/// The months of the year
#[derive(Debug, EnumIter, EnumCount, Copy, Clone)]
pub enum Month {
    /// [January](https://en.wikipedia.org/wiki/January) is the first month of the year
    January = 1,

    /// [February](https://en.wikipedia.org/wiki/February) is the second month of the year
    February = 2,

    /// [March](https://en.wikipedia.org/wiki/March) is the third month of the year
    March = 3,

    /// [April](https://en.wikipedia.org/wiki/April) is the fourth month of the year
    April = 4,

    /// [May](https://en.wikipedia.org/wiki/May) is the fifth month of the year
    May = 5,

    /// [June](https://en.wikipedia.org/wiki/June) is the sixth month of the year
    June = 6,

    /// [July](https://en.wikipedia.org/wiki/July) is the seventh month of the year
    July = 7,

    /// [August](https://en.wikipedia.org/wiki/August) is the eighth month of the year
    August = 8,

    /// [September](https://en.wikipedia.org/wiki/September) is the ninth month of the year
    September = 9,

    /// [October](https://en.wikipedia.org/wiki/October) is the tenth month of the year
    October = 10,

    /// [November](https://en.wikipedia.org/wiki/November) is the eleventh month of the year
    November = 11,

    /// [December](https://en.wikipedia.org/wiki/December) is the twelfth month of the year
    December = 12,
}
