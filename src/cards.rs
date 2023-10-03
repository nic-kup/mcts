use std::fmt;
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Suit {
    Diamond,
    Heart,
    Spade,
    Club,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Suit::Diamond => write!(f, "{}", "D"),
            Suit::Heart => write!(f, "{}", "H"),
            Suit::Spade => write!(f, "{}", "S"),
            Suit::Club => write!(f, "{}", "C"),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Card {
    pub value: u8,
    pub suit: Suit,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.suit, self.value)
    }
}

impl FromStr for Suit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "D" => Ok(Suit::Diamond),
            "H" => Ok(Suit::Heart),
            "S" => Ok(Suit::Spade),
            "C" => Ok(Suit::Club),
            _ => Err(format!("Invalid suit: {}", s)),
        }
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 2 && s.len() != 3 {
            return Err(format!("Invalid card string: {}", s));
        }
        let suit =
            s.chars().next().unwrap().to_string().parse::<Suit>()?;
        let value = s
            .chars()
            .nth(1)
            .unwrap()
            .to_digit(10)
            .ok_or(format!("Invalid value: {}", s))?;
        Ok(Card {
            suit,
            value: value as u8,
        })
    }
}

fn parse_cards(cards_str: &str) -> Result<Vec<Card>, String> {
    cards_str
        .split_whitespace()
        .map(str::parse::<Card>)
        .collect()
}

pub fn gen_full_deck() -> Vec<Card> {
    let mut all_cards: Vec<Card> = (1..14)
        .map(|x: u8| Card {
            value: x,
            suit: Suit::Diamond,
        })
        .collect();
    all_cards.extend((1..14).map(|x: u8| Card {
        value: x,
        suit: Suit::Heart,
    }));
    all_cards.extend((1..14).map(|x: u8| Card {
        value: x,
        suit: Suit::Spade,
    }));
    all_cards.extend((1..14).map(|x: u8| Card {
        value: x,
        suit: Suit::Club,
    }));
    return all_cards;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_gen_full_deck() {
        let mut all_cards: Vec<Card> = (1..14)
            .map(|x: u8| Card {
                value: x,
                suit: Suit::Diamond,
            })
            .collect();
        all_cards.extend((1..14).map(|x: u8| Card {
            value: x,
            suit: Suit::Heart,
        }));
        all_cards.extend((1..14).map(|x: u8| Card {
            value: x,
            suit: Suit::Spade,
        }));
        all_cards.extend((1..14).map(|x: u8| Card {
            value: x,
            suit: Suit::Club,
        }));
        assert_eq!(all_cards, gen_full_deck());
    }

    #[test]
    fn test_parse_cards() {
        let cards_str = "D1 H2 S3 C4";
        let cards = parse_cards(cards_str).unwrap();
        assert_eq!(
            cards,
            vec![
                Card {
                    suit: Suit::Diamond,
                    value: 1
                },
                Card {
                    suit: Suit::Heart,
                    value: 2
                },
                Card {
                    suit: Suit::Spade,
                    value: 3
                },
                Card {
                    suit: Suit::Club,
                    value: 4
                },
            ]
        );
    }
}
