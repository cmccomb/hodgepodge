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

impl AminoAcid {
    /// Looks up a three letter code ignoring ASCII case; does not trim whitespace.
    #[must_use]
    pub fn from_three_letter_code(code: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|value| value.three_letter_code().eq_ignore_ascii_case(code))
    }
}

dataset_enum! {
    /// The four canonical RNA bases; ambiguity codes and modified bases are excluded.
    pub enum RnaBase { Adenine => "Adenine", Cytosine => "Cytosine", Guanine => "Guanine", Uracil => "Uracil" }
}
impl RnaBase {
    /// Returns the uppercase RNA base symbol.
    #[must_use]
    pub const fn symbol(self) -> char {
        match self {
            Self::Adenine => 'A',
            Self::Cytosine => 'C',
            Self::Guanine => 'G',
            Self::Uracil => 'U',
        }
    }
    /// Returns the canonical Watson-Crick complement (A-U and C-G), excluding wobble pairs.
    #[must_use]
    pub const fn complement(self) -> Self {
        match self {
            Self::Adenine => Self::Uracil,
            Self::Uracil => Self::Adenine,
            Self::Cytosine => Self::Guanine,
            Self::Guanine => Self::Cytosine,
        }
    }
    /// Returns the matching coding-strand DNA base (U becomes T).
    #[must_use]
    pub const fn coding_dna(self) -> DnaBase {
        match self {
            Self::Adenine => DnaBase::Adenine,
            Self::Cytosine => DnaBase::Cytosine,
            Self::Guanine => DnaBase::Guanine,
            Self::Uracil => DnaBase::Thymine,
        }
    }
}
impl TryFrom<char> for RnaBase {
    type Error = crate::ParseEnumError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'A' => Ok(Self::Adenine),
            'C' => Ok(Self::Cytosine),
            'G' => Ok(Self::Guanine),
            'U' => Ok(Self::Uracil),
            _ => Err(crate::ParseEnumError::new("RnaBase")),
        }
    }
}
impl DnaBase {
    /// Converts a coding-strand base to RNA (T becomes U).
    /// For a template strand use the complement and account for antiparallel sequence orientation.
    #[must_use]
    pub const fn coding_rna(self) -> RnaBase {
        match self {
            Self::Adenine => RnaBase::Adenine,
            Self::Cytosine => RnaBase::Cytosine,
            Self::Guanine => RnaBase::Guanine,
            Self::Thymine => RnaBase::Uracil,
        }
    }
}

/// A codon's ordinary elongation meaning in an explicitly chosen genetic code.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum CodonMeaning {
    /// Incorporate the named amino acid.
    AminoAcid(AminoAcid),
    /// Terminate translation.
    Stop,
}

dataset_enum! {
    /// All 64 RNA codons, written in the 5-prime to 3-prime direction.
    /// Canonical Rust names are `PascalCase`; `sequence()` returns uppercase RNA.
    pub enum Codon {
        Uuu => "UUU",
        Uuc => "UUC",
        Uua => "UUA",
        Uug => "UUG",
        Ucu => "UCU",
        Ucc => "UCC",
        Uca => "UCA",
        Ucg => "UCG",
        Uau => "UAU",
        Uac => "UAC",
        Uaa => "UAA",
        Uag => "UAG",
        Ugu => "UGU",
        Ugc => "UGC",
        Uga => "UGA",
        Ugg => "UGG",
        Cuu => "CUU",
        Cuc => "CUC",
        Cua => "CUA",
        Cug => "CUG",
        Ccu => "CCU",
        Ccc => "CCC",
        Cca => "CCA",
        Ccg => "CCG",
        Cau => "CAU",
        Cac => "CAC",
        Caa => "CAA",
        Cag => "CAG",
        Cgu => "CGU",
        Cgc => "CGC",
        Cga => "CGA",
        Cgg => "CGG",
        Auu => "AUU",
        Auc => "AUC",
        Aua => "AUA",
        Aug => "AUG",
        Acu => "ACU",
        Acc => "ACC",
        Aca => "ACA",
        Acg => "ACG",
        Aau => "AAU",
        Aac => "AAC",
        Aaa => "AAA",
        Aag => "AAG",
        Agu => "AGU",
        Agc => "AGC",
        Aga => "AGA",
        Agg => "AGG",
        Guu => "GUU",
        Guc => "GUC",
        Gua => "GUA",
        Gug => "GUG",
        Gcu => "GCU",
        Gcc => "GCC",
        Gca => "GCA",
        Gcg => "GCG",
        Gau => "GAU",
        Gac => "GAC",
        Gaa => "GAA",
        Gag => "GAG",
        Ggu => "GGU",
        Ggc => "GGC",
        Gga => "GGA",
        Ggg => "GGG",
    }
}
impl Codon {
    /// Returns the three RNA bases in the 5-prime to 3-prime direction.
    #[must_use]
    pub const fn bases(self) -> [RnaBase; 3] {
        match self {
            Self::Uuu => [RnaBase::Uracil, RnaBase::Uracil, RnaBase::Uracil],
            Self::Uuc => [RnaBase::Uracil, RnaBase::Uracil, RnaBase::Cytosine],
            Self::Uua => [RnaBase::Uracil, RnaBase::Uracil, RnaBase::Adenine],
            Self::Uug => [RnaBase::Uracil, RnaBase::Uracil, RnaBase::Guanine],
            Self::Ucu => [RnaBase::Uracil, RnaBase::Cytosine, RnaBase::Uracil],
            Self::Ucc => [RnaBase::Uracil, RnaBase::Cytosine, RnaBase::Cytosine],
            Self::Uca => [RnaBase::Uracil, RnaBase::Cytosine, RnaBase::Adenine],
            Self::Ucg => [RnaBase::Uracil, RnaBase::Cytosine, RnaBase::Guanine],
            Self::Uau => [RnaBase::Uracil, RnaBase::Adenine, RnaBase::Uracil],
            Self::Uac => [RnaBase::Uracil, RnaBase::Adenine, RnaBase::Cytosine],
            Self::Uaa => [RnaBase::Uracil, RnaBase::Adenine, RnaBase::Adenine],
            Self::Uag => [RnaBase::Uracil, RnaBase::Adenine, RnaBase::Guanine],
            Self::Ugu => [RnaBase::Uracil, RnaBase::Guanine, RnaBase::Uracil],
            Self::Ugc => [RnaBase::Uracil, RnaBase::Guanine, RnaBase::Cytosine],
            Self::Uga => [RnaBase::Uracil, RnaBase::Guanine, RnaBase::Adenine],
            Self::Ugg => [RnaBase::Uracil, RnaBase::Guanine, RnaBase::Guanine],
            Self::Cuu => [RnaBase::Cytosine, RnaBase::Uracil, RnaBase::Uracil],
            Self::Cuc => [RnaBase::Cytosine, RnaBase::Uracil, RnaBase::Cytosine],
            Self::Cua => [RnaBase::Cytosine, RnaBase::Uracil, RnaBase::Adenine],
            Self::Cug => [RnaBase::Cytosine, RnaBase::Uracil, RnaBase::Guanine],
            Self::Ccu => [RnaBase::Cytosine, RnaBase::Cytosine, RnaBase::Uracil],
            Self::Ccc => [RnaBase::Cytosine, RnaBase::Cytosine, RnaBase::Cytosine],
            Self::Cca => [RnaBase::Cytosine, RnaBase::Cytosine, RnaBase::Adenine],
            Self::Ccg => [RnaBase::Cytosine, RnaBase::Cytosine, RnaBase::Guanine],
            Self::Cau => [RnaBase::Cytosine, RnaBase::Adenine, RnaBase::Uracil],
            Self::Cac => [RnaBase::Cytosine, RnaBase::Adenine, RnaBase::Cytosine],
            Self::Caa => [RnaBase::Cytosine, RnaBase::Adenine, RnaBase::Adenine],
            Self::Cag => [RnaBase::Cytosine, RnaBase::Adenine, RnaBase::Guanine],
            Self::Cgu => [RnaBase::Cytosine, RnaBase::Guanine, RnaBase::Uracil],
            Self::Cgc => [RnaBase::Cytosine, RnaBase::Guanine, RnaBase::Cytosine],
            Self::Cga => [RnaBase::Cytosine, RnaBase::Guanine, RnaBase::Adenine],
            Self::Cgg => [RnaBase::Cytosine, RnaBase::Guanine, RnaBase::Guanine],
            Self::Auu => [RnaBase::Adenine, RnaBase::Uracil, RnaBase::Uracil],
            Self::Auc => [RnaBase::Adenine, RnaBase::Uracil, RnaBase::Cytosine],
            Self::Aua => [RnaBase::Adenine, RnaBase::Uracil, RnaBase::Adenine],
            Self::Aug => [RnaBase::Adenine, RnaBase::Uracil, RnaBase::Guanine],
            Self::Acu => [RnaBase::Adenine, RnaBase::Cytosine, RnaBase::Uracil],
            Self::Acc => [RnaBase::Adenine, RnaBase::Cytosine, RnaBase::Cytosine],
            Self::Aca => [RnaBase::Adenine, RnaBase::Cytosine, RnaBase::Adenine],
            Self::Acg => [RnaBase::Adenine, RnaBase::Cytosine, RnaBase::Guanine],
            Self::Aau => [RnaBase::Adenine, RnaBase::Adenine, RnaBase::Uracil],
            Self::Aac => [RnaBase::Adenine, RnaBase::Adenine, RnaBase::Cytosine],
            Self::Aaa => [RnaBase::Adenine, RnaBase::Adenine, RnaBase::Adenine],
            Self::Aag => [RnaBase::Adenine, RnaBase::Adenine, RnaBase::Guanine],
            Self::Agu => [RnaBase::Adenine, RnaBase::Guanine, RnaBase::Uracil],
            Self::Agc => [RnaBase::Adenine, RnaBase::Guanine, RnaBase::Cytosine],
            Self::Aga => [RnaBase::Adenine, RnaBase::Guanine, RnaBase::Adenine],
            Self::Agg => [RnaBase::Adenine, RnaBase::Guanine, RnaBase::Guanine],
            Self::Guu => [RnaBase::Guanine, RnaBase::Uracil, RnaBase::Uracil],
            Self::Guc => [RnaBase::Guanine, RnaBase::Uracil, RnaBase::Cytosine],
            Self::Gua => [RnaBase::Guanine, RnaBase::Uracil, RnaBase::Adenine],
            Self::Gug => [RnaBase::Guanine, RnaBase::Uracil, RnaBase::Guanine],
            Self::Gcu => [RnaBase::Guanine, RnaBase::Cytosine, RnaBase::Uracil],
            Self::Gcc => [RnaBase::Guanine, RnaBase::Cytosine, RnaBase::Cytosine],
            Self::Gca => [RnaBase::Guanine, RnaBase::Cytosine, RnaBase::Adenine],
            Self::Gcg => [RnaBase::Guanine, RnaBase::Cytosine, RnaBase::Guanine],
            Self::Gau => [RnaBase::Guanine, RnaBase::Adenine, RnaBase::Uracil],
            Self::Gac => [RnaBase::Guanine, RnaBase::Adenine, RnaBase::Cytosine],
            Self::Gaa => [RnaBase::Guanine, RnaBase::Adenine, RnaBase::Adenine],
            Self::Gag => [RnaBase::Guanine, RnaBase::Adenine, RnaBase::Guanine],
            Self::Ggu => [RnaBase::Guanine, RnaBase::Guanine, RnaBase::Uracil],
            Self::Ggc => [RnaBase::Guanine, RnaBase::Guanine, RnaBase::Cytosine],
            Self::Gga => [RnaBase::Guanine, RnaBase::Guanine, RnaBase::Adenine],
            Self::Ggg => [RnaBase::Guanine, RnaBase::Guanine, RnaBase::Guanine],
        }
    }
    /// Constructs a codon from RNA bases in the 5-prime to 3-prime direction.
    #[must_use]
    pub fn from_bases(bases: [RnaBase; 3]) -> Self {
        // ALL is exhaustive over the Cartesian product of the four RNA bases.
        fn index(base: RnaBase) -> usize {
            match base {
                RnaBase::Uracil => 0,
                RnaBase::Cytosine => 1,
                RnaBase::Adenine => 2,
                RnaBase::Guanine => 3,
            }
        }
        Self::ALL[index(bases[0]) * 16 + index(bases[1]) * 4 + index(bases[2])]
    }
    /// Returns the uppercase three-letter RNA sequence.
    #[must_use]
    pub const fn sequence(self) -> &'static str {
        self.label()
    }
    /// Parses exactly three RNA letters, ignoring ASCII case; T and ambiguity codes are rejected.
    #[must_use]
    pub fn from_sequence(sequence: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|codon| codon.sequence().eq_ignore_ascii_case(sequence))
    }
    /// Returns the ordinary elongation meaning in NCBI standard genetic code, table 1.
    /// Initiation context and other genetic codes require a separate interpretation.
    #[must_use]
    pub const fn standard_meaning(self) -> CodonMeaning {
        match self {
            Self::Uuu | Self::Uuc => CodonMeaning::AminoAcid(AminoAcid::Phenylalanine),
            Self::Uua | Self::Uug | Self::Cuu | Self::Cuc | Self::Cua | Self::Cug => {
                CodonMeaning::AminoAcid(AminoAcid::Leucine)
            }
            Self::Ucu | Self::Ucc | Self::Uca | Self::Ucg | Self::Agu | Self::Agc => {
                CodonMeaning::AminoAcid(AminoAcid::Serine)
            }
            Self::Uau | Self::Uac => CodonMeaning::AminoAcid(AminoAcid::Tyrosine),
            Self::Uaa | Self::Uag | Self::Uga => CodonMeaning::Stop,
            Self::Ugu | Self::Ugc => CodonMeaning::AminoAcid(AminoAcid::Cysteine),
            Self::Ugg => CodonMeaning::AminoAcid(AminoAcid::Tryptophan),
            Self::Ccu | Self::Ccc | Self::Cca | Self::Ccg => {
                CodonMeaning::AminoAcid(AminoAcid::Proline)
            }
            Self::Cau | Self::Cac => CodonMeaning::AminoAcid(AminoAcid::Histidine),
            Self::Caa | Self::Cag => CodonMeaning::AminoAcid(AminoAcid::Glutamine),
            Self::Cgu | Self::Cgc | Self::Cga | Self::Cgg | Self::Aga | Self::Agg => {
                CodonMeaning::AminoAcid(AminoAcid::Arginine)
            }
            Self::Auu | Self::Auc | Self::Aua => CodonMeaning::AminoAcid(AminoAcid::Isoleucine),
            Self::Aug => CodonMeaning::AminoAcid(AminoAcid::Methionine),
            Self::Acu | Self::Acc | Self::Aca | Self::Acg => {
                CodonMeaning::AminoAcid(AminoAcid::Threonine)
            }
            Self::Aau | Self::Aac => CodonMeaning::AminoAcid(AminoAcid::Asparagine),
            Self::Aaa | Self::Aag => CodonMeaning::AminoAcid(AminoAcid::Lysine),
            Self::Guu | Self::Guc | Self::Gua | Self::Gug => {
                CodonMeaning::AminoAcid(AminoAcid::Valine)
            }
            Self::Gcu | Self::Gcc | Self::Gca | Self::Gcg => {
                CodonMeaning::AminoAcid(AminoAcid::Alanine)
            }
            Self::Gau | Self::Gac => CodonMeaning::AminoAcid(AminoAcid::AsparticAcid),
            Self::Gaa | Self::Gag => CodonMeaning::AminoAcid(AminoAcid::GlutamicAcid),
            Self::Ggu | Self::Ggc | Self::Gga | Self::Ggg => {
                CodonMeaning::AminoAcid(AminoAcid::Glycine)
            }
        }
    }
    /// Whether NCBI table 1 marks this codon as a possible initiation codon.
    /// Actual initiation is context-dependent; marked starts encode methionine at initiation.
    #[must_use]
    pub const fn is_standard_start(self) -> bool {
        matches!(self, Self::Uug | Self::Cug | Self::Aug)
    }
    /// Returns the codon of the reverse-complement RNA sequence in 5-prime to 3-prime order.
    #[must_use]
    pub fn reverse_complement(self) -> Self {
        let [first, second, third] = self.bases();
        Self::from_bases([third.complement(), second.complement(), first.complement()])
    }
}
impl AminoAcid {
    /// Enumerates codons encoding this residue during elongation in NCBI table 1.
    #[must_use]
    pub fn standard_codons(self) -> impl DoubleEndedIterator<Item = Codon> {
        Codon::ALL
            .iter()
            .copied()
            .filter(move |codon| codon.standard_meaning() == CodonMeaning::AminoAcid(self))
    }
}
