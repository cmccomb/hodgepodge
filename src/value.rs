use std::fmt;

/// An integer outside the values represented by a numeric dataset.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EnumValueError {
    enum_name: &'static str,
    value: u8,
}

impl EnumValueError {
    pub(crate) const fn new(enum_name: &'static str, value: u8) -> Self {
        Self { enum_name, value }
    }

    /// Returns the target enum's type name.
    #[must_use]
    pub const fn enum_name(self) -> &'static str {
        self.enum_name
    }

    /// Returns the rejected integer.
    #[must_use]
    pub const fn value(self) -> u8 {
        self.value
    }
}

impl fmt::Display for EnumValueError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid {} value", self.value, self.enum_name)
    }
}

impl std::error::Error for EnumValueError {}
