//! Simulate two fair dice with a caller-supplied, seeded generator.
use hodgepodge::DiceFace;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let mut sums = [0_u32; 13];
    for _ in 0..10_000 {
        let first: DiceFace = rng.random();
        let second: DiceFace = rng.random();
        sums[usize::from(first.ordinal() + second.ordinal())] += 1;
    }
    for (sum, count) in sums.iter().enumerate().skip(2) {
        println!("{sum:2}: {count:4} rolls");
    }
}
