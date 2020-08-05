//! A module for time enums

use strum_macros::EnumIter;

/// The days of the week
#[derive(Debug, EnumIter)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

/// The months of the year
#[derive(Debug, EnumIter)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
