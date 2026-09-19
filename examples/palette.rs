//! Iterate over a dataset and group aliases by their shared RGB value.
use hodgepodge::{IntoEnumIterator, CSS};
use std::collections::BTreeMap;

fn main() {
    let mut palette: BTreeMap<u32, Vec<&str>> = BTreeMap::new();
    for color in CSS::iter() {
        palette.entry(color.rgb()).or_default().push(color.as_str());
    }
    for (rgb, names) in palette {
        println!("#{rgb:06x}: {}", names.join(", "));
    }
}
