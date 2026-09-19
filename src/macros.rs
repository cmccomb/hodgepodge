//! Shared implementations keep every dataset's basic API consistent.

/// Declare an enum and its allocation-free name and parsing operations.
macro_rules! dataset_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($(#[$variant_meta:meta])* $variant:ident $(= $value:expr)?),* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "strum", derive(strum_macros::EnumIter, strum_macros::EnumCount))]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub enum $name {
            $($(#[$variant_meta])* $variant $(= $value)?),*
        }

        impl $name {
            /// Returns the canonical Rust variant name, without allocating.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant)),*
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.pad(self.as_str())
            }
        }

        impl std::str::FromStr for $name {
            type Err = crate::ParseEnumError;

            fn from_str(input: &str) -> Result<Self, Self::Err> {
                $(if input.eq_ignore_ascii_case(stringify!($variant)) {
                    return Ok(Self::$variant);
                })*
                Err(crate::ParseEnumError::new(stringify!($name)))
            }
        }
    };
}

/// Keep color identity separate from its RGB value so aliases can agree.
macro_rules! color_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($(#[$variant_meta:meta])* $variant:ident = $rgb:expr),* $(,)?
        }
    ) => {
        dataset_enum! {
            $(#[$meta])*
            pub enum $name {
                $($(#[$variant_meta])* $variant),*
            }
        }

        impl $name {
            /// Returns the packed 24-bit RGB value (`0xRRGGBB`).
            ///
            /// Use this method instead of casting the enum to an integer.
            #[must_use]
            pub const fn rgb(self) -> u32 {
                match self {
                    $(Self::$variant => $rgb),*
                }
            }
        }

        impl std::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.pad_integral(true, "0x", &format!("{:06x}", self.rgb()))
            }
        }
    };
}
