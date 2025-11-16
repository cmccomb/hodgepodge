// Import hodgepodge. Be sure to import all if you want to use iters.
use hodgepodge::{Element, IntoEnumIterator};

fn main() {
    // How many elements are there?
    let element_count = Element::iter().count();
    println!("There are {element_count} elements");
}
