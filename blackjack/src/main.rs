use crate::HandSum::Hard;
use crate::HandSum::Soft;
use crate::Suit::*;
use crate::Value::*;
use std::collections::HashSet;
use std::ops::Add;

const TWENTY_ONE: ValueType = 21;
const LOW_ACE_VALUE: ValueType = 1;
const HIGH_ACE_VALUE: ValueType = 11;
const FACE_CARD_VALUE: ValueType = 10;

fn main() {}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
}

type ValueType = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Value {
    Number(ValueType),
    Jack,
    Queen,
    King,
    Ace,
}

impl Value {
    fn of(self, suit: Suit) -> Card {
        Card { suit, value: self }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Card {
    suit: Suit,
    value: Value,
}

impl Card {
    fn as_sum(&self) -> HandSum {
        match self.value {
            Value::Number(val) => Hard(val),
            Value::Ace => Soft(HIGH_ACE_VALUE),
            _ => Hard(FACE_CARD_VALUE),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum HandSum {
    Soft(ValueType),
    Hard(ValueType),
}

impl HandSum {
    fn unwrap(&self) -> ValueType {
        match self {
            Hard(v) => *v,
            Soft(v) => *v,
        }
    }
}

trait Deck {
    fn draw(&mut self) -> Option<&Card>;
}

struct FrenchDeck {
    cards: Vec<Card>,
}

impl FrenchDeck {
    fn new() -> Self {
        let suits = vec![Hearts, Spades, Diamonds, Clubs];
        let cards = suits
            .iter()
            .flat_map(|suit| {
                let values = vec![
                    Number(2),
                    Number(3),
                    Number(4),
                    Number(5),
                    Number(6),
                    Number(7),
                    Number(8),
                    Number(9),
                    Number(10),
                    King,
                    Queen,
                    Jack,
                    Ace,
                ];
                values.into_iter().map(|value| Card { suit: *suit, value })
            })
            .collect();
        Self { cards }
    }
}

impl Deck for FrenchDeck {
    fn draw(&mut self) -> Option<&Card> {
        // Draw something random, which I can't do because I need the random crate
        None
    }
}

struct Shoe {
    decks:Vec<Box<dyn Deck>>
}

impl Deck for Shoe {
    fn draw(&mut self)->Option<&Card> {
        // Draw something random, which I can't do because I need the random crate
        None
    }
}

impl Shoe {
    fn new(decks:Vec<Box<dyn Deck>>) -> Self {
Self{decks}
    }

    fn new_with(count:usize, deckBuilder:Fn<() => dyn Deck>)-> Self {
        let decks = (0..count).map(|_| deckBuilder()).collect();
        Self::new(decks)
    }
}

impl Add for HandSum {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Hard(lval), Hard(rval)) => Hard(lval + rval),
            (Soft(lval), _) => {
                let soft_sum = lval + other.unwrap();
                if soft_sum > TWENTY_ONE {
                    Hard(lval - (HIGH_ACE_VALUE - LOW_ACE_VALUE)) + other
                } else {
                    Soft(soft_sum)
                }
            }
            (Hard(_), Soft(_)) => other + self,
        }
    }
}

fn sum_hand(cards: &HashSet<Card>) -> HandSum {
    cards
        .iter()
        .map(|c| c.as_sum())
        .fold(Hard(0), |acc, new| acc + new)
}

#[test]
fn test_hand_summing_hard_values() {
    assert_eq!(Hard(5) + Hard(4), Hard(9));
    assert_eq!(Hard(15) + Hard(4), Hard(19));
    assert_eq!(Hard(18) + Hard(4), Hard(22));
}

#[test]
fn test_hand_summation_one_soft_value() {
    assert_eq!(Hard(5) + Soft(15), Soft(20));
    assert_eq!(Soft(15) + Hard(5), Soft(20));
    assert_eq!(Soft(19) + Hard(5), Hard(14));
}

#[test]
fn test_hand_summation_two_soft_values() {
    assert_eq!(Soft(11) + Soft(8), Soft(19));
    assert_eq!(Soft(19) + Soft(8), Soft(17));
    assert_eq!(Soft(19) + Soft(18), Hard(17));
}

#[test]
fn test_summing_hands() {
    let test_cases: Vec<(HashSet<Card>, HandSum)> = vec![
        (
            vec![Number(7).of(Hearts), Number(10).of(Spades)]
                .into_iter()
                .collect(),
            Hard(17),
        ),
        (
            vec![Ace.of(Spades), Number(8).of(Hearts)]
                .into_iter()
                .collect(),
            Soft(19),
        ),
        (
            vec![Ace.of(Spades), Number(8).of(Hearts), King.of(Hearts)]
                .into_iter()
                .collect(),
            Hard(19),
        ),
    ];
    for (cards, sum) in test_cases {
        assert_eq!(sum_hand(&cards), sum);
    }
}

#[test]
fn build_french_deck() {
    let deck = FrenchDeck::new();
    assert_eq!(deck.cards.len(), 52);
    assert_eq!(
        deck.cards.iter().filter(|card| card.suit == Hearts).count(),
        13
    );
    assert_eq!(
        deck.cards.iter().filter(|card| card.suit == Spades).count(),
        13
    );
    assert_eq!(
        deck.cards
            .iter()
            .filter(|card| card.suit == Diamonds)
            .count(),
        13
    );
    assert_eq!(
        deck.cards.iter().filter(|card| card.suit == Clubs).count(),
        13
    );

    assert_eq!(deck.cards.iter().collect::<HashSet<_>>().len(), 52);
}
