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
        let suit_char = s.chars().next().unwrap();
        let suit = suit_char.to_string().parse::<Suit>()?;
        let value_str = &s[1..];

        let value = match value_str {
            "J" => Ok(11),
            "Q" => Ok(12),
            "K" => Ok(13),
            _ => value_str
                .parse::<u8>()
                .map_err(|_| format!("Invalid value: {}", value_str)),
        }?;
        if value > 13 || value < 1 {
            return Err(format!(
                "Value too small or too large: value = {}",
                value
            ));
        }
        Ok(Card { suit, value })
    }
}

pub fn parse_cards(cards_str: &str) -> Result<Vec<Card>, String> {
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
    fn test_faulty_parse_cards() {
        let wrong_cards: &str = "D1 H2 C14";
        assert_eq!(
            parse_cards(wrong_cards),
            Err("Value too small or too large: value = 14".to_string())
        );
    }

    #[test]
    fn test_parse_cards() {
        let cards_str: &str = "D1 H2 S3 C4 H10 C12";
        let cards: Vec<Card> = parse_cards(cards_str).unwrap();
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
                Card {
                    suit: Suit::Heart,
                    value: 10,
                },
                Card {
                    suit: Suit::Club,
                    value: 12,
                }
            ]
        );
    }
}
