use hodgepodge;
use strum::IntoEnumIterator;

fn main() {
    for member in hodgepodge::science::Element::iter() {
        println!("{:?}", member);
    }
}