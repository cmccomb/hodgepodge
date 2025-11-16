// Import hodgepodge.
use hodgepodge::RGB;

fn main() {
    // Print hex codes
    let blue = RGB::Blue;
    let red = RGB::Red;
    let green = RGB::Green;
    println!("#{blue:06x}, #{red:06x}, and #{green:06x} are the RGB colors");
}
