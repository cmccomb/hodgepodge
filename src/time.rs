//! Timekeeping datasets including weekdays and months.
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

// Enables the optional iterator and variant-count derives.
#[cfg(feature = "strum")]
use strum_macros::{EnumCount, EnumIter};

/// The days of the week
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

/// The four meteorological seasons in calendar order.
///
/// # Examples
/// ```
/// use hodgepodge::Season;
///
/// let vacation = Season::Summer;
/// assert_eq!(vacation as u8, 3);
/// ```
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Season {
    /// [Winter](https://en.wikipedia.org/wiki/Winter) is the first season of the year.
    Winter = 1,

    /// [Spring](https://en.wikipedia.org/wiki/Spring_(season)) is the second season of the year.
    Spring = 2,

    /// [Summer](https://en.wikipedia.org/wiki/Summer) is the third season of the year.
    Summer = 3,

    /// [Fall](https://en.wikipedia.org/wiki/Autumn) (autumn) is the fourth season of the year.
    Fall = 4,
}

/// Fiscal quarters in chronological order.
///
/// # Examples
/// ```
/// use hodgepodge::Quarter;
///
/// let reporting_period = Quarter::Q4;
/// assert_eq!(reporting_period as u8, 4);
/// ```
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "strum", derive(EnumIter, EnumCount))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Quarter {
    /// The first quarter of the fiscal year.
    Q1 = 1,

    /// The second quarter of the fiscal year.
    Q2 = 2,

    /// The third quarter of the fiscal year.
    Q3 = 3,

    /// The fourth quarter of the fiscal year.
    Q4 = 4,
}

#[cfg(all(test, feature = "strum"))]
mod tests {
    use super::{Quarter, Season};
    use crate::{EnumCount, IntoEnumIterator};

    #[test]
    fn seasons_iterate_in_calendar_order() {
        let observed: Vec<u8> = Season::iter().map(|season| season as u8).collect();
        assert_eq!(observed, vec![1, 2, 3, 4]);
    }

    #[test]
    fn seasons_count_matches_four() {
        assert_eq!(<Season as EnumCount>::COUNT, 4);
    }

    #[test]
    fn quarters_iterate_in_chronological_order() {
        let observed: Vec<u8> = Quarter::iter().map(|quarter| quarter as u8).collect();
        assert_eq!(observed, vec![1, 2, 3, 4]);
    }

    #[test]
    fn quarters_count_matches_four() {
        assert_eq!(<Quarter as EnumCount>::COUNT, 4);
    }
}
