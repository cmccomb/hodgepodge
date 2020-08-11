// Import hodgepodge. Be sure to import all if you want to use iters.
use hodgepodge::*;

fn main() {
    // Iterate
    for member in Element::iter() {
        // All the elements!
        println!("{:?} is element {:?}", member.clone(), member as i32);
    }
}