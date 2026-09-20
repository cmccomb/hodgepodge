//! Follow relationships and inspect metadata without optional features.
use hodgepodge::{AminoAcid, Bone, DnaBase, GeologicEpoch, ParseEnumError, SkeletalDivision};

fn main() -> Result<(), ParseEnumError> {
    let axial = Bone::ALL
        .iter()
        .filter(|bone| bone.division() == SkeletalDivision::Axial)
        .count();
    println!(
        "{} adult bones: {axial} axial, {} appendicular",
        Bone::COUNT,
        Bone::COUNT - axial
    );
    let epoch = GeologicEpoch::Holocene;
    let period = epoch.period();
    let era = period.era();
    println!("{epoch} → {period} → {era} → {}", era.eon());
    let reverse_complement: String = "AGTC"
        .chars()
        .rev()
        .map(|c| DnaBase::try_from(c).map(|base| base.complement().symbol()))
        .collect::<Result<_, _>>()?;
    println!("AGTC reverse complement: {reverse_complement}");
    for amino_acid in AminoAcid::ALL {
        println!(
            "{}: {} ({})",
            amino_acid.label(),
            amino_acid.three_letter_code(),
            amino_acid.one_letter_code()
        );
    }
    Ok(())
}
