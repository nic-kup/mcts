use crate::cards::{gen_full_deck, parse_cards, Card, Suit};
use rand::{rngs::ThreadRng, seq::SliceRandom};

struct GameState {
    turn: u8, // suboptimal solution
    hands: Vec<Vec<Card>>,
    played: Vec<Card>,
    freed: Vec<Vec<Suit>>,
    trick: Vec<Card>,
}

struct KnownGameState {
    turn: u8, // suboptimal solution
    hand: Vec<Card>,
    played: Vec<Card>,
    freed: Vec<Vec<Suit>>,
    trick: Vec<Card>,
}

struct InformationSet {
    parent: Option<Box<InformationSet>>,
    children: Vec<Box<InformationSet>>,
    move_made: Option<Move>,
    visit_count: u32,
    value_sum: f32,
    known_game_state: KnownGameState,
}

impl KnownGameState {
    fn determinize(&self, mut rng: ThreadRng) -> GameState {
        let mut unkown_cards: Vec<Card> = gen_full_deck()
            .into_iter()
            .filter(|x: &Card| {
                !self.played.contains(&x)
                    && !self.trick.contains(&x)
                    && !self.hand.contains(&x)
            })
            .collect();
        let num_players = self.freed.len();
        unkown_cards.shuffle(&mut rng);
        let mut hands: Vec<Vec<Card>> = vec![Vec::new(); num_players];
        hands[self.turn as usize] = self.hand.clone();

        let mut player_order: Vec<usize> = (0..num_players).collect();
        player_order.rotate_left(self.turn as usize);

        for _i in 0..self.trick.len() {
            player_order.pop(); // Remove players who already played
        }

        let mut player_idx = 0;
        while !unkown_cards.is_empty() {
            let player = player_order[player_idx];
            hands[player].push(unkown_cards.pop().unwrap());
            player_idx = (player_idx + 1) & player_order.len();
        }

        let game_state = GameState {
            turn: self.turn,
            hands: hands,
            played: self.played.clone(),
            freed: self.freed.clone(),
            trick: self.trick.clone(),
        };
        return game_state;
    }
}

//     fn simulate(&self) -> f32 {
//         return 0.0;
//     }
// }

// impl GameState {
//     fn simulate(&self) -> f32 {
//         return 0.0;
//     }
// }

enum Move {
    Passed(Vec<Card>),
    PlayedCard(Card),
    Nothing,
}

fn backpropagate(_node: &mut InformationSet, _result: f32) {
    // Update node.visit_count and node.value_sum
    // Recurse on node.parent (if it exists)
}

fn ucb1(
    _parent_visit_count: u32,
    _node_visit_count: u32,
    _node_value_sum: f32,
) -> f32 {
    return 1.0;
}

// fn tree_policy(node: &InformationSet) -> &InformationSet {
// Select child nodes using UCB1 until a leaf node is reached
// }

// fn expand(_node: &mut InformationSet) {
// Generate all possible moves, create child nodes, and add them to node.children
// }

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;
    #[test]
    fn test_determinize() {
        // let _rng: ThreadRng = thread_rng();
        // let known_game_state = KnownGameState {
        //     turn: 0,
        //     hand: parse_cards("H13, H12, H11")
        // }
    }
}
