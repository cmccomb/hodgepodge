//! Timekeeping datasets including weekdays and months.
#![allow(missing_docs)]
#![allow(clippy::missing_docs_in_private_items)]

numeric_enum! {
    /// The days of the week
    pub enum Day {
        /// [Monday](https://en.wikipedia.org/wiki/Monday) is the first day of the week
        Monday = 1 => "Monday",

        /// [Tuesday](https://en.wikipedia.org/wiki/Tuesday) is the second day of the week
        Tuesday = 2 => "Tuesday",

        /// [Wednesday](https://en.wikipedia.org/wiki/Wednesday) is the third day of the week
        Wednesday = 3 => "Wednesday",

        /// [Thursday](https://en.wikipedia.org/wiki/Thursday) is the fourth day of the week
        Thursday = 4 => "Thursday",

        /// [Friday](https://en.wikipedia.org/wiki/Friday) is the fifth day of the week
        Friday = 5 => "Friday",

        /// [Saturday](https://en.wikipedia.org/wiki/Saturday) is the sixth day of the week
        Saturday = 6 => "Saturday",

        /// [Sunday](https://en.wikipedia.org/wiki/Sunday) is the seventh day of the week
        Sunday = 7 => "Sunday",
    }
}

numeric_enum! {
    /// The months of the year
    pub enum Month {
        /// [January](https://en.wikipedia.org/wiki/January) is the first month of the year
        January = 1 => "January",

        /// [February](https://en.wikipedia.org/wiki/February) is the second month of the year
        February = 2 => "February",

        /// [March](https://en.wikipedia.org/wiki/March) is the third month of the year
        March = 3 => "March",

        /// [April](https://en.wikipedia.org/wiki/April) is the fourth month of the year
        April = 4 => "April",

        /// [May](https://en.wikipedia.org/wiki/May) is the fifth month of the year
        May = 5 => "May",

        /// [June](https://en.wikipedia.org/wiki/June) is the sixth month of the year
        June = 6 => "June",

        /// [July](https://en.wikipedia.org/wiki/July) is the seventh month of the year
        July = 7 => "July",

        /// [August](https://en.wikipedia.org/wiki/August) is the eighth month of the year
        August = 8 => "August",

        /// [September](https://en.wikipedia.org/wiki/September) is the ninth month of the year
        September = 9 => "September",

        /// [October](https://en.wikipedia.org/wiki/October) is the tenth month of the year
        October = 10 => "October",

        /// [November](https://en.wikipedia.org/wiki/November) is the eleventh month of the year
        November = 11 => "November",

        /// [December](https://en.wikipedia.org/wiki/December) is the twelfth month of the year
        December = 12 => "December",
    }
}

dataset_enum! {
    /// The four meteorological seasons in Northern Hemisphere calendar order.
    ///
    /// This order is a teaching convention and does not apply to both hemispheres.
    ///
    /// # Examples
    /// ```
    /// use hodgepodge::Season;
    ///
    /// let vacation = Season::Summer;
    /// assert_eq!(vacation as u8, 3);
    /// ```
    pub enum Season {
        /// [Winter](https://en.wikipedia.org/wiki/Winter) is the first season of the year.
        Winter = 1 => "Winter",

        /// [Spring](https://en.wikipedia.org/wiki/Spring_(season)) is the second season of the year.
        Spring = 2 => "Spring",

        /// [Summer](https://en.wikipedia.org/wiki/Summer) is the third season of the year.
        Summer = 3 => "Summer",

        /// [Fall](https://en.wikipedia.org/wiki/Autumn) (autumn) is the fourth season of the year.
        Fall = 4 => "Fall",
    }
}

dataset_enum! {
    /// Fiscal quarters in chronological order.
    ///
    /// # Examples
    /// ```
    /// use hodgepodge::Quarter;
    ///
    /// let reporting_period = Quarter::Q4;
    /// assert_eq!(reporting_period as u8, 4);
    /// ```
    pub enum Quarter {
        /// The first quarter of the fiscal year.
        Q1 = 1 => "Quarter 1",

        /// The second quarter of the fiscal year.
        Q2 = 2 => "Quarter 2",

        /// The third quarter of the fiscal year.
        Q3 = 3 => "Quarter 3",

        /// The fourth quarter of the fiscal year.
        Q4 = 4 => "Quarter 4",
    }
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

impl Day {
    /// Returns the Monday-first weekday number, from 1 to 7.
    /// Use `Day::try_from(number)` for checked conversion back.
    #[must_use]
    pub const fn number(self) -> u8 {
        self as u8
    }
}

impl Month {
    /// Returns the calendar month number, from 1 to 12.
    /// Use `Month::try_from(number)` for checked conversion back.
    #[must_use]
    pub const fn number(self) -> u8 {
        self as u8
    }
}

impl Day {
    /// Returns the three-letter English abbreviation.
    #[must_use]
    pub const fn abbreviation(self) -> &'static str {
        match self {
            Self::Monday => "Mon",
            Self::Tuesday => "Tue",
            Self::Wednesday => "Wed",
            Self::Thursday => "Thu",
            Self::Friday => "Fri",
            Self::Saturday => "Sat",
            Self::Sunday => "Sun",
        }
    }
}

impl Month {
    /// Returns the three-letter English abbreviation.
    #[must_use]
    pub const fn abbreviation(self) -> &'static str {
        match self {
            Self::January => "Jan",
            Self::February => "Feb",
            Self::March => "Mar",
            Self::April => "Apr",
            Self::May => "May",
            Self::June => "Jun",
            Self::July => "Jul",
            Self::August => "Aug",
            Self::September => "Sep",
            Self::October => "Oct",
            Self::November => "Nov",
            Self::December => "Dec",
        }
    }
}

impl Day {
    /// Whether this is Saturday or Sunday under the common five-day workweek convention.
    #[must_use]
    pub const fn is_weekend(self) -> bool {
        matches!(self, Self::Saturday | Self::Sunday)
    }
}

impl Month {
    /// Returns the month length in a non-leap Gregorian year.
    #[must_use]
    pub const fn days_in_common_year(self) -> u8 {
        match self {
            Self::January
            | Self::March
            | Self::May
            | Self::July
            | Self::August
            | Self::October
            | Self::December => 31,
            Self::February => 28,
            Self::April | Self::June | Self::September | Self::November => 30,
        }
    }
}

impl Season {
    /// Returns Northern Hemisphere meteorological months, in seasonal order.
    #[must_use]
    pub const fn months(self) -> [Month; 3] {
        match self {
            Self::Winter => [Month::December, Month::January, Month::February],
            Self::Spring => [Month::March, Month::April, Month::May],
            Self::Summer => [Month::June, Month::July, Month::August],
            Self::Fall => [Month::September, Month::October, Month::November],
        }
    }
}

impl Quarter {
    /// Returns the fiscal quarter number (1–4), independent of fiscal start month.
    #[must_use]
    pub const fn number(self) -> u8 {
        self as u8
    }
}

checked_u8_enum!(Quarter, number);

impl Day {
    /// Looks up a abbreviation ignoring ASCII case; does not trim whitespace.
    #[must_use]
    pub fn from_abbreviation(code: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|value| value.abbreviation().eq_ignore_ascii_case(code))
    }
}

impl Month {
    /// Looks up a abbreviation ignoring ASCII case; does not trim whitespace.
    #[must_use]
    pub fn from_abbreviation(code: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|value| value.abbreviation().eq_ignore_ascii_case(code))
    }
}

dataset_enum! {
    /// Northern and Southern Hemisphere meteorological-season conventions.
    /// Local tropical wet/dry seasons are outside this classification.
    pub enum Hemisphere { Northern => "Northern", Southern => "Southern" }
}

impl Day {
    /// Returns the following day, wrapping Sunday to Monday.
    #[must_use]
    pub const fn next(self) -> Self {
        Self::ALL[self.number() as usize % 7]
    }
    /// Returns the preceding day, wrapping Monday to Sunday.
    #[must_use]
    pub const fn previous(self) -> Self {
        Self::ALL[(self.number() as usize + 5) % 7]
    }
}

impl Month {
    /// Returns the following month, wrapping December to January.
    #[must_use]
    pub const fn next(self) -> Self {
        Self::ALL[self.number() as usize % 12]
    }
    /// Returns the preceding month, wrapping January to December.
    #[must_use]
    pub const fn previous(self) -> Self {
        Self::ALL[(self.number() as usize + 10) % 12]
    }
    /// Returns the length in a proleptic Gregorian year (astronomical year numbering).
    #[must_use]
    pub const fn days_in_year(self, year: i32) -> u8 {
        if matches!(self, Self::February) && is_gregorian_leap_year(year) {
            29
        } else {
            self.days_in_common_year()
        }
    }
    /// Returns the meteorological season under the specified hemisphere convention.
    #[must_use]
    pub const fn season(self, hemisphere: Hemisphere) -> Season {
        let north = match self {
            Self::December | Self::January | Self::February => Season::Winter,
            Self::March | Self::April | Self::May => Season::Spring,
            Self::June | Self::July | Self::August => Season::Summer,
            Self::September | Self::October | Self::November => Season::Fall,
        };
        match hemisphere {
            Hemisphere::Northern => north,
            Hemisphere::Southern => north.opposite(),
        }
    }
    /// Returns the quarter with an explicit fiscal start month; January gives calendar quarters.
    #[must_use]
    pub const fn quarter(self, fiscal_start: Self) -> Quarter {
        Quarter::ALL[((self.number() as usize + 12 - fiscal_start.number() as usize) % 12) / 3]
    }
}

/// Whether a year is leap in the proleptic Gregorian calendar, including year zero.
#[must_use]
pub const fn is_gregorian_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

impl Season {
    /// Returns the season six months away in the four-season convention.
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::Winter => Self::Summer,
            Self::Summer => Self::Winter,
            Self::Spring => Self::Fall,
            Self::Fall => Self::Spring,
        }
    }
    /// Returns meteorological months for the specified hemisphere.
    #[must_use]
    pub const fn months_in(self, hemisphere: Hemisphere) -> [Month; 3] {
        match hemisphere {
            Hemisphere::Northern => self.months(),
            Hemisphere::Southern => self.opposite().months(),
        }
    }
}

impl Quarter {
    /// Returns months in fiscal order for an explicit fiscal start month.
    #[must_use]
    pub const fn months(self, fiscal_start: Month) -> [Month; 3] {
        let first = (fiscal_start.number() as usize - 1 + (self.number() as usize - 1) * 3) % 12;
        [
            Month::ALL[first],
            Month::ALL[(first + 1) % 12],
            Month::ALL[(first + 2) % 12],
        ]
    }
}
