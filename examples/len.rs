// Import hodgepodge. Be sure to import all if you want to use iters.
use hodgepodge::*;

fn main() {
    // How many elements are there?
    println!("There are {:?} elements", Element::iter().count());
}