use mcts::cards::gen_full_deck;


fn main() {
    let full_deck = gen_full_deck();
    for card in full_deck {
        println!("{}", card);
    }
}
