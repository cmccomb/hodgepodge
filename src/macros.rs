//! Shared implementations keep every dataset's basic API consistent.

/// Declare an enum and its allocation-free name and parsing operations.
macro_rules! dataset_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($(#[$variant_meta:meta])* $variant:ident $(= $value:expr)? $(=> $label:literal)?),* $(,)?
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
            /// All variants in declaration order, available without features.
            pub const ALL: &'static [Self] = &[$(Self::$variant),*];

            /// The number of variants, available without features.
            pub const COUNT: usize = Self::ALL.len();

            /// Returns a human-readable label, separate from parsing and Serde names.
            #[must_use]
            pub const fn label(self) -> &'static str {
                match self {
                    $(Self::$variant => dataset_enum!(@label $variant $(, $label)?)),*
                }
            }

            /// Returns the canonical Rust variant name, without allocating.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => stringify!($variant)),*
                }
            }
        }

        impl crate::Dataset for $name {
            const INFO: crate::DatasetInfo = Self::INFO;
            const ALL: &'static [Self] = Self::ALL;
            const COUNT: usize = Self::COUNT;
            fn as_str(self) -> &'static str { self.as_str() }
            fn label(self) -> &'static str { self.label() }
        }

        #[cfg(feature = "rand")]
        impl rand::distr::Distribution<$name> for rand::distr::StandardUniform {
            fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> $name {
                $name::ALL[rng.random_range(0..$name::COUNT)]
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
    (@label $variant:ident, $label:literal) => { $label };
    (@label $variant:ident) => { stringify!($variant) };
}

// Opt-in conversion for enums whose explicit discriminants represent u8 values.
macro_rules! numeric_enum {
    ($(#[$meta:meta])* pub enum $name:ident {
        $($(#[$variant_meta:meta])* $variant:ident = $value:literal $(=> $label:literal)?),* $(,)?
    }) => {
        dataset_enum! {
            $(#[$meta])* pub enum $name {
                $($(#[$variant_meta])* $variant = $value $(=> $label)?),*
            }
        }
        impl TryFrom<u8> for $name {
            type Error = crate::EnumValueError;

            fn try_from(value: u8) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok(Self::$variant),)*
                    _ => Err(crate::EnumValueError::new(stringify!($name), value)),
                }
            }
        }
    };
}

/// Keep color identity separate from its RGB value so aliases can agree.
macro_rules! color_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($(#[$variant_meta:meta])* $variant:ident = $rgb:expr $(=> $label:literal)?),* $(,)?
        }
    ) => {
        dataset_enum! {
            $(#[$meta])*
            pub enum $name {
                $($(#[$variant_meta])* $variant $(=> $label)?),*
            }
        }

        impl $name {
            /// Returns red, green, and blue channel values, each from 0 to 255.
            #[must_use]
            pub const fn rgb_channels(self) -> [u8; 3] {
                let [_, red, green, blue] = self.rgb().to_be_bytes();
                [red, green, blue]
            }

            /// Returns every palette entry sharing an exact packed RGB value.
            /// Aliases retain their identity; values above 0xFFFFFF have no matches.
            #[must_use]
            pub fn matching_rgb(rgb: u32) -> impl DoubleEndedIterator<Item = Self> {
                Self::ALL.iter().copied().filter(move |color| color.rgb() == rgb)
            }

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

// Checked conversions use documented values, never incidental declaration indexes.
macro_rules! checked_u8_enum {
    ($name:ident, $method:ident) => {
        impl TryFrom<u8> for $name {
            type Error = crate::EnumValueError;
            fn try_from(value: u8) -> Result<Self, Self::Error> {
                Self::ALL
                    .iter()
                    .copied()
                    .find(|item| item.$method() == value)
                    .ok_or_else(|| crate::EnumValueError::new(stringify!($name), value))
            }
        }
    };
}

macro_rules! group_members {
    ($parent:ident, $method:ident, $child:ident, $relation:ident) => {
        impl $parent {
            /// Enumerates matching members of this library's documented selection.
            #[must_use]
            pub fn $method(self) -> impl DoubleEndedIterator<Item = $child> {
                $child::ALL
                    .iter()
                    .copied()
                    .filter(move |member| member.$relation() == self)
            }
        }
    };
}
