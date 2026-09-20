//! Literary datasets with explicit source-specific conventions.
#![allow(missing_docs)]

dataset_enum! {
    /// The nine circles in Dante's Inferno, in descent order.
    /// Anger includes wrath and sullenness; Greed includes avarice and prodigality.
    /// The vestibule, subdivisions, Purgatorio, and Paradiso are excluded.
    pub enum InfernoCircle {
        Limbo = 1 => "Limbo",
        Lust = 2 => "Lust",
        Gluttony = 3 => "Gluttony",
        Greed = 4 => "Greed",
        Anger = 5 => "Anger",
        Heresy = 6 => "Heresy",
        Violence = 7 => "Violence",
        Fraud = 8 => "Fraud",
        Treachery = 9 => "Treachery",
    }
}

impl InfernoCircle {
    /// Returns the circle number, 1 (Limbo) through 9 (Treachery).
    #[must_use]
    pub const fn number(self) -> u8 {
        self as u8
    }
}

checked_u8_enum!(InfernoCircle, number);
