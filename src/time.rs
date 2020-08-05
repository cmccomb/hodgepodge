//! A module for time enums

use strum_macros::EnumIter;

/// The days of the week
#[derive(Debug, EnumIter)]
pub enum Day {
    /// [](https://en.wikipedia.org/wiki/) is the first day of the week
    Monday,

    /// [Tuesday](https://en.wikipedia.org/wiki/Tuesday) is the second day of the week
    Tuesday,

    /// [Wednesday](https://en.wikipedia.org/wiki/Wednesday) is the third day of the week
    Wednesday,

    /// [Thursday](https://en.wikipedia.org/wiki/Thursday) is the fourth day of the week
    Thursday,

    /// [Friday](https://en.wikipedia.org/wiki/Friday) is the fifth day of the week
    Friday,

    /// [Saturday](https://en.wikipedia.org/wiki/Saturday) is the sixth day of the week
    Saturday,

    /// [Sunday](https://en.wikipedia.org/wiki/Sunday) is the seventh day of the week
    Sunday,
}

/// The months of the year
#[derive(Debug, EnumIter)]
pub enum Month {
    /// [January](https://en.wikipedia.org/wiki/January) is the first month of the year
    January,

    /// [February](https://en.wikipedia.org/wiki/February) is the second month of the year
    February,

    /// [March](https://en.wikipedia.org/wiki/March) is the third month of the year
    March,

    /// [April](https://en.wikipedia.org/wiki/April) is the fourth month of the year
    April,

    /// [May](https://en.wikipedia.org/wiki/May) is the fifth month of the year
    May,

    /// [June](https://en.wikipedia.org/wiki/June) is the sixth month of the year
    June,

    /// [July](https://en.wikipedia.org/wiki/July) is the seventh month of the year
    July,

    /// [August](https://en.wikipedia.org/wiki/August) is the eighth month of the year
    August,

    /// [September](https://en.wikipedia.org/wiki/September) is the ninth month of the year
    September,

    /// [October](https://en.wikipedia.org/wiki/October) is the tenth month of the year
    October,

    /// [November](https://en.wikipedia.org/wiki/November) is the eleventh month of the year
    November,

    /// [December](https://en.wikipedia.org/wiki/December) is the twelfth month of the year
    December,
}
