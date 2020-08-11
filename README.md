[![Build Status](https://travis-ci.org/cmccomb/hodgepodge.svg?branch=master)](https://travis-ci.org/cmccomb/hodgepodge)

# About
The name says it all - this crate is just a hodgepodge of potentially useful enums.

# Examples
Usage is pretty simple. Import, and use to your heart's desire.
```
use hodgepodge::*;

fn main() {
    println!("{:?}, {:?}, and {:?} are RGB colors", RGB::Blue, RGB::Red, RGB::Green);
}
```
This library uses [`strum`](https://crates.io/crates/strum) and [`strum_macros`](https://crates.io/crates/strum_macros), so you can do things like this:
```
use hodgepodge::*;

fn main() {
    for member in Element::iter() {
        println!("{:?} is element {:?}", member.clone(), member as i32);
    }
}
```
And this:
```
use hodgepodge::*;

fn main() {
    println!("There are {:?} elements", Element::iter().count());
}
```
