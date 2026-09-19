//! Errors for parsing dataset names.

/// A string did not match a variant of the requested dataset.
///
/// Parsing accepts canonical Rust variant names with ASCII case ignored.
/// Whitespace, spaces between words, numeric values, and alternate spellings
/// are not accepted. Call `trim()` explicitly if your input contains whitespace.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ParseEnumError {
    /// The dataset that rejected the input.
    enum_name: &'static str,
}

impl ParseEnumError {
    /// Creates an error for the named dataset.
    pub(crate) const fn new(enum_name: &'static str) -> Self {
        Self { enum_name }
    }

    /// Returns the name of the dataset that rejected the input.
    #[must_use]
    pub const fn enum_name(self) -> &'static str {
        self.enum_name
    }
}

impl std::fmt::Display for ParseEnumError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown {} variant", self.enum_name)
    }
}

impl std::error::Error for ParseEnumError {}
