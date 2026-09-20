//! DNA bases and the twenty standard proteinogenic amino acids.
#![allow(missing_docs)]

dataset_enum! {
    /// The four canonical DNA bases; excludes RNA uracil and ambiguity codes.
    pub enum DnaBase {
        Adenine => "Adenine",
        Cytosine => "Cytosine",
        Guanine => "Guanine",
        Thymine => "Thymine",
    }
}

impl DnaBase {
    /// Returns the complementary base (A–T, C–G).
    #[must_use]
    pub const fn complement(self) -> Self {
        match self {
            Self::Adenine => Self::Thymine,
            Self::Thymine => Self::Adenine,
            Self::Cytosine => Self::Guanine,
            Self::Guanine => Self::Cytosine,
        }
    }
}

impl DnaBase {
    /// Returns the uppercase one-letter base symbol.
    #[must_use]
    pub const fn symbol(self) -> char {
        match self {
            Self::Adenine => 'A',
            Self::Cytosine => 'C',
            Self::Guanine => 'G',
            Self::Thymine => 'T',
        }
    }
}

dataset_enum! {
    /// The twenty standard amino acids, not whole proteins.
    /// Selenocysteine, pyrrolysine, ambiguous residues, and stop codes are excluded.
    pub enum AminoAcid {
        Alanine => "Alanine",
        Arginine => "Arginine",
        Asparagine => "Asparagine",
        AsparticAcid => "Aspartic Acid",
        Cysteine => "Cysteine",
        GlutamicAcid => "Glutamic Acid",
        Glutamine => "Glutamine",
        Glycine => "Glycine",
        Histidine => "Histidine",
        Isoleucine => "Isoleucine",
        Leucine => "Leucine",
        Lysine => "Lysine",
        Methionine => "Methionine",
        Phenylalanine => "Phenylalanine",
        Proline => "Proline",
        Serine => "Serine",
        Threonine => "Threonine",
        Tryptophan => "Tryptophan",
        Tyrosine => "Tyrosine",
        Valine => "Valine",
    }
}

impl AminoAcid {
    /// Returns the standard uppercase one-letter code.
    #[must_use]
    pub const fn one_letter_code(self) -> char {
        match self {
            Self::Alanine => 'A',
            Self::Arginine => 'R',
            Self::Asparagine => 'N',
            Self::AsparticAcid => 'D',
            Self::Cysteine => 'C',
            Self::GlutamicAcid => 'E',
            Self::Glutamine => 'Q',
            Self::Glycine => 'G',
            Self::Histidine => 'H',
            Self::Isoleucine => 'I',
            Self::Leucine => 'L',
            Self::Lysine => 'K',
            Self::Methionine => 'M',
            Self::Phenylalanine => 'F',
            Self::Proline => 'P',
            Self::Serine => 'S',
            Self::Threonine => 'T',
            Self::Tryptophan => 'W',
            Self::Tyrosine => 'Y',
            Self::Valine => 'V',
        }
    }
}

impl AminoAcid {
    /// Returns the standard three-letter abbreviation.
    #[must_use]
    pub const fn three_letter_code(self) -> &'static str {
        match self {
            Self::Alanine => "Ala",
            Self::Arginine => "Arg",
            Self::Asparagine => "Asn",
            Self::AsparticAcid => "Asp",
            Self::Cysteine => "Cys",
            Self::GlutamicAcid => "Glu",
            Self::Glutamine => "Gln",
            Self::Glycine => "Gly",
            Self::Histidine => "His",
            Self::Isoleucine => "Ile",
            Self::Leucine => "Leu",
            Self::Lysine => "Lys",
            Self::Methionine => "Met",
            Self::Phenylalanine => "Phe",
            Self::Proline => "Pro",
            Self::Serine => "Ser",
            Self::Threonine => "Thr",
            Self::Tryptophan => "Trp",
            Self::Tyrosine => "Tyr",
            Self::Valine => "Val",
        }
    }
}

/// Parses an ASCII one-letter code, ignoring ASCII case.
impl TryFrom<char> for DnaBase {
    type Error = crate::ParseEnumError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'A' => Ok(Self::Adenine),
            'C' => Ok(Self::Cytosine),
            'G' => Ok(Self::Guanine),
            'T' => Ok(Self::Thymine),
            _ => Err(crate::ParseEnumError::new("DnaBase")),
        }
    }
}

/// Parses an ASCII one-letter code, ignoring ASCII case.
impl TryFrom<char> for AminoAcid {
    type Error = crate::ParseEnumError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'A' => Ok(Self::Alanine),
            'R' => Ok(Self::Arginine),
            'N' => Ok(Self::Asparagine),
            'D' => Ok(Self::AsparticAcid),
            'C' => Ok(Self::Cysteine),
            'E' => Ok(Self::GlutamicAcid),
            'Q' => Ok(Self::Glutamine),
            'G' => Ok(Self::Glycine),
            'H' => Ok(Self::Histidine),
            'I' => Ok(Self::Isoleucine),
            'L' => Ok(Self::Leucine),
            'K' => Ok(Self::Lysine),
            'M' => Ok(Self::Methionine),
            'F' => Ok(Self::Phenylalanine),
            'P' => Ok(Self::Proline),
            'S' => Ok(Self::Serine),
            'T' => Ok(Self::Threonine),
            'W' => Ok(Self::Tryptophan),
            'Y' => Ok(Self::Tyrosine),
            'V' => Ok(Self::Valine),
            _ => Err(crate::ParseEnumError::new("AminoAcid")),
        }
    }
}
