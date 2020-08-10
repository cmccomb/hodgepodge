use hodgepodge::*;

fn main() {
    for member in Element::iter() {
        println!("{:?} is element {:?}", member.clone(), member as i32);
    }
}