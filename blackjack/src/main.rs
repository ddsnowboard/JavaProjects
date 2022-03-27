use crate::HandSum::Hard;
use crate::HandSum::Soft;
use crate::Suit::*;
use crate::Value::*;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::collections::HashSet;
use std::iter::from_fn;
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

trait SingleDeck {
    fn cards(&self) -> &Vec<Card>;
    fn cards_mut(&mut self) -> &mut Vec<Card>;
}

trait Deck {
    fn draw(&mut self) -> Option<Card>;
    fn is_empty(&self) -> bool;
}

impl<T: SingleDeck> Deck for T {
    fn draw(&mut self) -> Option<Card> {
        let cards = self.cards_mut();
        if cards.is_empty() {
            None
        } else {
            let between = Uniform::from(0..cards.len());
            let mut rng = thread_rng();

            let index = between.sample(&mut rng);
            Some(cards.swap_remove(index))
        }
    }

    fn is_empty(&self) -> bool {
        self.cards().len() == 0
    }
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

impl SingleDeck for FrenchDeck {
    fn cards(&self) -> &Vec<Card> {
        &self.cards
    }
    fn cards_mut(&mut self) -> &mut Vec<Card> {
        &mut self.cards
    }
}

struct Shoe {
    decks: Vec<Box<dyn Deck>>,
}

impl Deck for Shoe {
    fn draw(&mut self) -> Option<Card> {
        let mut non_empty_decks: Vec<_> = self.decks.iter_mut().filter(|d| !d.is_empty()).collect();
        if non_empty_decks.is_empty() {
            None
        } else {
            let between = Uniform::from(0..non_empty_decks.len());
            let mut rng = thread_rng();

            let index = between.sample(&mut rng);
            non_empty_decks[index].draw()
        }
    }

    fn is_empty(&self) -> bool {
        self.decks.iter().all(|d| d.is_empty())
    }
}

impl Shoe {
    fn new(decks: Vec<Box<dyn Deck>>) -> Self {
        Self { decks }
    }

    // This is 'static just to mean that it can't be a short-lived pointer
    fn new_with<T: Deck + 'static>(count: usize, deck_builder: fn() -> T) -> Self {
        let decks = (0..count)
            .map(|_| Box::new(deck_builder()) as Box<dyn Deck>)
            .collect();
        Self::new(decks)
    }
}

#[test]
fn test_building_shoe() {
    let decks: Vec<Box<dyn Deck>> = vec![Box::new(FrenchDeck::new()), Box::new(FrenchDeck::new())];
    let mut shoe = Shoe::new(decks);
    assert_eq!(shoe.decks.len(), 2);
    assert_eq!(shoe.is_empty(), false);
    let cards: Vec<Card> = from_fn(|| shoe.draw()).collect();
    assert!(shoe.is_empty());
    assert_eq!(cards.len(), 52 * 2);
    assert_eq!(cards.into_iter().collect::<HashSet<_>>().len(), 52);
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

trait BlackjackStrategy{
    fn play(hand:Vec<&// I can't decide what exactly to give the players. Hmm.
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
