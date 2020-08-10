use hodgepodge::*;

fn main() {
    for member in Element::iter() {
        println!("{:?}", member);
    }
}