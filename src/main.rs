use mcts::cards::gen_full_deck;
use rand::random;

fn main() {
    let full_deck = gen_full_deck();
    for card in full_deck {
        println!("{}", card);
    }
}
