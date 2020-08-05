[![Build Status](https://travis-ci.org/cmccomb/hodgepodge.svg?branch=master)](https://travis-ci.org/cmccomb/hodgepodge)

# About
The name says it all - this crate just a hodgepodge or potentially useful enums.

# Examples
Usage is pretty simple. Import, and use to your heart's desire.
```
use hodgepodge::colors::RGB;

fn main() {
    println!("{:?}, {:?}, and {:?}", RGB::Blue, RGB::Red, RGB::Green);
}
```
If you want to iterate through the members of the enum, do this:
```
use hodgepodge;
use strum::IntoEnumIterator;

fn main() {
    for member in hodgepodge::science::Element::iter() {
        println!("{:?}", member);
    }
}
```
