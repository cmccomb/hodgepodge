//! Shuffle once, then deal four non-overlapping five-card hands.
use hodgepodge::shuffled_deck;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

fn main() {
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    let deck = shuffled_deck(&mut rng);
    for (player, hand) in deck.chunks_exact(5).take(4).enumerate() {
        println!("Player {}:", player + 1);
        for card in hand {
            println!("  {} of {}", card.rank.label(), card.suit.label());
        }
    }
    println!("{} cards remain", deck.len() - 20);
}
