//! Store a deterministic enum fixture as JSON and recover the same names.
use hodgepodge::{DiceFace, CSS};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Fixture {
    rolls: Vec<DiceFace>,
    swatches: Vec<CSS>,
}

fn main() -> Result<(), serde_json::Error> {
    let fixture = Fixture {
        rolls: vec![DiceFace::One, DiceFace::Six, DiceFace::Three],
        swatches: vec![CSS::Aqua, CSS::Cyan, CSS::Fuchsia],
    };
    let json = serde_json::to_string_pretty(&fixture)?;
    let recovered: Fixture = serde_json::from_str(&json)?;
    assert_eq!(recovered, fixture);
    assert_ne!(recovered.swatches[0], recovered.swatches[1]);
    assert_eq!(recovered.swatches[0].rgb(), recovered.swatches[1].rgb());
    println!("{json}");
    Ok(())
}
