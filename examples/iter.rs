// Import hodgepodge. Be sure to import all if you want to use iters.
use hodgepodge::{Element, IntoEnumIterator};

fn main() {
    // Iterate
    for member in Element::iter() {
        // All the elements!
        let atomic_number = member as i32;
        println!("{member:?} is element {atomic_number}");
    }
}
