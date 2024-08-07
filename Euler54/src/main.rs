use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::fmt;


#[derive(PartialEq, Clone, Copy, Hash, Debug)]
enum Suit { Diamonds, Clubs, Spades, Hearts }

fn suit_from_string(s: &str) -> Suit {
    match s {
        "D" => Suit::Diamonds,
        "C" => Suit::Clubs,
        "H" => Suit::Hearts,
        "S" => Suit::Spades,
        _ => panic!()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Spades => write!(f, "Spades"),
            Suit::Clubs => write!(f, "Clubs"),
        }
    }
}

#[derive(PartialEq, Clone, Copy, Ord, PartialOrd, Eq, Hash, Debug)]
enum Value {
    Zero = 0,
    Two = 2,
    Three,
    Four, 
    Five, 
    Six, 
    Seven, 
    Eight, 
    Nine, 
    Ten, 
    Jack,
    Queen, 
    King, 
    Ace 
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", *self as u32)
    }
}

impl Value {
    fn next(&self) -> Self {
        match *self {
            Value::Ace => Value::Two, 
            Value::Zero => panic!("You tried to increment Zero!"),
            x => Self::from_u32((x as u32) + 1)
        }
    }

    fn from_u32(i: u32) -> Self {
        match i {
            0 => Value::Zero,
            2 => Value::Two,
            3 => Value::Three,
            4 => Value::Four,
            5 => Value::Five,
            6 => Value::Six,
            7 => Value::Seven,
            8 => Value::Eight,
            9 => Value::Nine,
            10 => Value::Ten,
            11 => Value::Jack,
            12 => Value::Queen,
            13 => Value::King,
            14 => Value::Ace,
            _ => panic!("You gave a bad value!")
        }
    }
}

fn value_from_string(s: &str) -> Value {
    match s {
        "2" => Value::Two,
        "3" => Value::Three,
        "4" => Value::Four,
        "5" => Value::Five,
        "6" => Value::Six,
        "7" => Value::Seven,
        "8" => Value::Eight,
        "9" => Value::Nine,
        "T" => Value::Ten,
        "J" => Value::Jack,
        "Q" => Value::Queen,
        "K" => Value::King,
        "A" => Value::Ace,
        _ => panic!()
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
struct Card {
    suit: Suit,
    value: Value
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.value, self.suit)
    }
}

#[derive(PartialEq, Clone, Copy)]
struct Hand {
    cards: [Card; 5]
}

impl Hand {
    fn new(mut cards: Vec<Card>) -> Hand {
        cards.sort_by_key(|el| el.value);
        let mut new_cards = [Card { value: Value::Ace, suit: Suit::Spades }; 5];
        for (idx, card) in cards.into_iter().enumerate() {
            new_cards[idx] = card;
        }
        Hand { cards: new_cards }
    }

    // These all return an option with the high card if there is one
    fn has_royal_flush(&self) -> Option<&Card> {
        let necessary_cards: [Value; 5] = [Value::Ace, Value::King, Value::Queen, Value::Jack, Value::Ten];
        let has_cards = necessary_cards.into_iter().all(|v| self.contains(v).is_some());
        if !has_cards {
            return None;
        }
        if self.have_same_suit() {
            self.contains(&Value::Ace)
        } else {
            None
        }
    }

    fn have_same_suit(&self) -> bool {
        let starting_suit = self.cards[0].suit;
        for card in self.cards.iter() {
            if card.suit != starting_suit {
                return false;
            }
        }
        true
    }

    fn has_flush(&self) -> Option<&Card> {
        if self.have_same_suit() {
            Some(&self.cards[4])
        } else {
            None
        }
    }

    fn has_straight(&self) -> Option<&Card> {
        let paradigm_straight: [i32; 5] = [1,2,3,4,5];

        let values = self.cards.iter().map(|x| x.value as i32);
        let mut deltas = paradigm_straight.iter().zip(values)
            .map(|(a, b)| b - a);
        let is_straight = deltas.all(|x| x == self.cards[0].value as i32 - paradigm_straight[0]);

        if is_straight {
            Some(&self.cards[4])
        } else {
            None
        }
    }

    fn has_straight_flush(&self) -> Option<&Card> {
        if !self.have_same_suit() {
            return None;
        }

        self.has_straight()
    }

    fn make_counts(&self) -> HashMap<Value, u32> {
        let mut hash = HashMap::new();
        for card in self.cards.iter() {
            let e = hash.entry(card.value).or_insert(0);
            *e += 1;
        }
        hash
    }

    fn has_n_of_a_kind(&self, n: u32) -> Option<&Card> {
        // Refactor this to return the highest one all the time so you can have has_high_card =
        // has_n_of_a_kind(1)
        let counts = self.make_counts();
        let mut counts: Vec<_> = counts.into_iter().filter(|x| x.1 == n).collect();
        counts.sort();
        let counts = counts;

        for &(key, value) in counts.iter().rev() {
            if value == n {
                return self.contains(&key);
            }
        }
        None
    }

    // This is guaranteed to return the big pair second and the small one first
    fn has_two_pair(&self) -> Option<(&Card, &Card)> {
        let hash = self.make_counts();
        let mut first: Option<&Card> = None;
        for (key, value) in hash.iter() {
            if *value == 2 {
                match first {
                    Some(first_card) => {
                        let second_card = self.contains(key).unwrap();
                        if first_card.value > second_card.value {
                            return Some((second_card, first_card));
                        } else {
                            return Some((first_card, second_card));
                        }
                    }, 
                    None => {
                        first = self.contains(key);
                    }
                }
            }
        }
        None
    }

    fn has_4_of_a_kind(&self) -> Option<&Card> {
        self.has_n_of_a_kind(4)
    }

    fn has_3_of_a_kind(&self) -> Option<&Card> {
        self.has_n_of_a_kind(3)
    }

    fn has_pair(&self) -> Option<&Card> {
        self.has_n_of_a_kind(2)
    }

    fn has_high_card(&self) -> Option<&Card> {
        self.has_n_of_a_kind(1)
    }

    fn contains(&self, value: &Value) -> Option<&Card> {
        for card in self.cards.iter() {
            if card.value == *value {
                return Some(&card)
            }
        }
        None
    }

    // This is guaranteed to give the card with 3 first and with 2 second
    fn has_full_house(&self) -> Option<(&Card, &Card)> {
        if let (Some(c1), Some(c2)) = (self.has_3_of_a_kind(), self.has_pair()) {
            Some((c1, c2))
        } else {
            None
        }
    }
}

impl Card {
    fn new(string: &str) -> Card {
        // Can be 3 if we have a 10
        assert!(string.len() == 2);
        let (left, right) = string.split_at(1);
        let value = value_from_string(left);
        let suit = suit_from_string(right);
        Card { value: value, suit: suit }
    }
}

fn player1_wins(p1: &Hand, p2: &Hand) -> bool {
    let pick_high_card = || {
        p1.has_high_card().map(|c| c.value).unwrap_or(Value::Zero) > p2.has_high_card().map(|c| c.value).unwrap_or(Value::Zero)
    };

    let pick_winner = | p1_card: Option<&Card>, p2_card: Option<&Card> | {
        match (p1_card, p2_card) {
            (Some(_), None) => true,
            (None, Some(_)) => false,
            (Some(c), Some(p)) => if c.value == p.value { pick_high_card() } else { c.value > p.value }, 
            (None, None) => pick_high_card()
        }
    };

    if p1.has_royal_flush().is_some() || p2.has_royal_flush().is_some() {
        p1.has_royal_flush().is_some()
    } else if p1.has_straight_flush().is_some() || p2.has_straight_flush().is_some() {
        pick_winner(p1.has_straight_flush(), p2.has_straight_flush())
    } else if p1.has_4_of_a_kind().is_some() || p2.has_4_of_a_kind().is_some() {
        pick_winner(p1.has_4_of_a_kind(), p2.has_4_of_a_kind())
    } else if p1.has_full_house().is_some() || p2.has_full_house().is_some() {
        // This is because the zeroth slot holds the three cards, which breaks ties
        pick_winner(p1.has_full_house().map(|t| t.0), p2.has_full_house().map(|t| t.0))
    } else if p1.has_flush().is_some() || p2.has_flush().is_some() {
        pick_winner(p1.has_flush(), p2.has_flush())
    } else if p1.has_straight().is_some() || p2.has_straight().is_some() {
        pick_winner(p1.has_straight(), p2.has_straight())
    } else if p1.has_3_of_a_kind().is_some() || p2.has_3_of_a_kind().is_some() {
        pick_winner(p1.has_3_of_a_kind(), p2.has_3_of_a_kind())
    } else if p1.has_two_pair().is_some() || p2.has_two_pair().is_some() {
        let pairs = (p1.has_two_pair(), p2.has_two_pair());
        match pairs {
            (Some((_, p1_big)), Some((_, p2_big))) if p1_big.value > p2_big.value => true, 
            (Some((_, p1_big)), Some((_, p2_big))) if p1_big.value < p2_big.value => false, 
            (Some((p1_small, _)), Some((p2_small, _))) if p1_small.value > p2_small.value => true , 
            (Some((p1_small, _)), Some((p2_small, _))) if p1_small.value < p2_small.value => false, 
            (Some(_), None)  => true, 
            (None, Some(_)) => false, 
            _ => pick_winner(None, None)
        }
    } else if p1.has_pair().is_some() || p2.has_pair().is_some() {
        pick_winner(p1.has_pair(), p2.has_pair())
    } else {
        pick_winner(None, None)
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut win_count = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let (h1, h2) = parse_line_of_hands(&line);
            if player1_wins(&h1, &h2) {
                win_count += 1;
            }
        }
    }
    println!("p1 won {} times", win_count);
}


fn parse_line_of_hands(s: &str) -> (Hand, Hand) {
    let mut cards = s.split_whitespace();
    let mut p1_cards = vec![];
    for _ in 0..5 {
        if let Some(c) = cards.next() {
            p1_cards.push(c);
        }
    }
    let mut p2_cards = vec![];
    for _ in 0..5 {
        if let Some(c) = cards.next() {
            p2_cards.push(c);
        }
    }
    let p1_cards = p1_cards.into_iter().map(|s| Card::new(s)).collect();
    let p2_cards = p2_cards.into_iter().map(|s| Card::new(s)).collect();
    let h1 = Hand::new(p1_cards);
    let h2 = Hand::new(p2_cards);
    (h1, h2)
}

#[test]
fn test_parsing() {
    let c = Card::new("5H");
    assert_eq!(c.value, Value::Five);
    assert_eq!(c.suit, Suit::Hearts);


    let c = Card::new("TD");
    assert_eq!(c.value, Value::Ten);
    assert_eq!(c.suit, Suit::Diamonds);
}

#[test]
fn test_hand_sorting() {
    let cards = vec!["5H", "4H", "2H", "6H", "3H"].into_iter().map(|s| Card::new(s)).collect();
    let hand = Hand::new(cards);
    let cards = [Card::new("2H"), Card::new("3H"), Card::new("4H"), Card::new("5H"), Card::new("6H")];
    assert_eq!(hand.cards, cards);
}

#[test]
fn test_pair_finding() {
    let (h1, _) = parse_line_of_hands("2H 3H 4H 2D 5S 5D 5D 5D 5D 6S");
    if let Some(c) = h1.has_pair() {
        assert_eq!(c.value, Value::Two);
    } else {
        panic!();
    }

    let (h1, _) = parse_line_of_hands("2H 2S 4H 2D 5S 5D 5D 5D 5D 6S");
    assert_eq!(h1.has_pair(), None);
}

#[test]
fn test_full_house_finding() {
    let (h1, _) = parse_line_of_hands("2H 3H 2D 3D 2S 5D 5D 5D 5D 6S");
    if let Some((c1, c2)) = h1.has_full_house() {
        if c1 == c2 {
            panic!("has_full_house() returned the same card twice");
        }
        assert!(c1.value == Value::Two || c2.value == Value::Two);
        assert!(c1.value == Value::Three || c2.value == Value::Three);
    } else {
        panic!();
    }
}

#[test] 
fn test_high_card() {
    let (h1, _) = parse_line_of_hands("2H QH QD KD KS 5D 5D 5D 5D 6S");
    if let Some(c1) = h1.has_high_card() {
        assert_eq!(c1.value, Value::Two);
    } else {
        panic!("High card doesn't work");
    }
}

#[test] 
fn test_pair_returns_highest() {
    let (h1, _) = parse_line_of_hands("2H QH QD KD KS 5D 5D 5D 5D 6S");
    if let Some(c1) = h1.has_pair() {
        assert_eq!(c1.value, Value::King);
    }
}

#[test]
fn test_higher_pair_wins() {
    let (h1, h2) = parse_line_of_hands("5H 5C 6S 7S KD 2C 3S 8S 8D TD");
    assert!(!player1_wins(&h1, &h2));
}

#[test] 
fn test_high_card_only() {
    let (h1, h2) = parse_line_of_hands("5D 8C 9S JS AC 2C 5C 7D 8S QH");
    assert!(player1_wins(&h1, &h2));
}

#[test] 
fn test_order_of_hands() {
    let (h1, h2) = parse_line_of_hands("2D 9C AS AH AC 3D 6D 7D TD QD");
    if let Some(c) = h1.has_3_of_a_kind() {
        assert!(c.value == Value::Ace);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_flush() {
        assert!(c.suit == Suit::Diamonds);
    } else {
        panic!();
    }
    assert!(!player1_wins(&h1, &h2));
}

#[test] 
fn test_high_card_wins_with_equal_pair() {
    let (h1, h2) = parse_line_of_hands("4D 6S 9H QH QC 3D 6D 7H QD QS");
    assert!(player1_wins(&h1, &h2));
}

#[test] 
fn test_order_of_hands_with_full_house() {
    let (h1, h2) = parse_line_of_hands("2H 2D 4C 4D 4S 3C 3D 3S 9S 9D");
    if let Some((c1, c2)) = h1.has_full_house() {
        assert!(c1.value == Value::Four);
        assert!(c2.value == Value::Two);
    } else {
        panic!();
    }

    if let Some((c1, c2)) = h2.has_full_house() {
        assert!(c1.value == Value::Three);
        assert!(c2.value == Value::Nine);
    } else {
        panic!();
    }
    assert!(player1_wins(&h1, &h2));
    assert!(!player1_wins(&h2, &h1));
}

#[test] 
fn test_order_of_flushes() {
    let (h1, h2) = parse_line_of_hands("2H 3H 4H 5H 7H 6D 7D 9D TD QD");
    if let Some(c) = h1.has_flush() {
        assert!(c.suit == Suit::Hearts);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_flush() {
        assert!(c.suit == Suit::Diamonds);
    } else {
        panic!();
    }
    assert!(!player1_wins(&h1, &h2));
}

#[test] 
fn test_convoluted_flush() {
    let (h1, h2) = parse_line_of_hands("2H 3H 4H 5H TH 6D 7D 8D 9D TD");
    if let Some(c) = h1.has_flush() {
        assert!(c.suit == Suit::Hearts);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_flush() {
        assert!(c.suit == Suit::Diamonds);
    } else {
        panic!();
    }

    assert!(!player1_wins(&h1, &h2));
    assert!(player1_wins(&h2, &h1));
}

#[test] 
fn test_boring_high_card() {
    let (h1, h2) = parse_line_of_hands("8C TS KC 9H 4S 7D 2S 5D 3S AC");
    if let Some(c) = h1.has_high_card() {
        assert!(c.suit == Suit::Clubs);
        assert!(c.value == Value::King);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_high_card() {
        assert!(c.suit == Suit::Clubs);
        assert!(c.value == Value::Ace);
    } else {
        panic!();
    }

    assert!(!player1_wins(&h1, &h2));
    assert!(player1_wins(&h2, &h1));
}

#[test] 
fn test_order_of_pairs() {
    let (h1, h2) = parse_line_of_hands("KC 8D QD 6D KH 5C 7H 9D 3D 9C");
    if let Some(c) = h1.has_pair() {
        assert!(c.value == Value::King);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_pair() {
        assert!(c.value == Value::Nine);
    } else {
        panic!();
    }

    assert!(player1_wins(&h1, &h2));
    assert!(!player1_wins(&h2, &h1));
}

#[test] 
fn test_order_of_hands_flush_to_higher_pair() {
    let (h1, h2) = parse_line_of_hands("2D 4H 5D QD QC 6H 6H 9H QH 8H");
    if let Some(c) = h1.has_high_card() {
        assert!(c.value == Value::Five);
        assert!(c.suit == Suit::Diamonds);
    } else {
        panic!();
    }

    if let Some(c) = h1.has_pair() {
        assert!(c.value == Value::Queen);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_pair() {
        assert!(c.value == Value::Six);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_flush() {
        assert!(c.suit == Suit::Hearts);
    } else {
        panic!();
    }

    assert!(!player1_wins(&h1, &h2));
    assert!(player1_wins(&h2, &h1));
}

#[test] 
fn test_high_card_after_pair() {
    let (h1, h2) = parse_line_of_hands("2D 4H 5D QD QC 6H 3D 9H QH QS");
    if let Some(c) = h1.has_high_card() {
        assert!(c.value == Value::Five);
        assert!(c.suit == Suit::Diamonds);
    } else {
        panic!();
    }

    if let Some(c) = h1.has_pair() {
        assert!(c.value == Value::Queen);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_pair() {
        assert!(c.value == Value::Queen);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_high_card() {
        assert!(c.value == Value::Nine);
        assert!(c.suit == Suit::Hearts);
    } else {
        panic!();
    }

    assert!(!player1_wins(&h1, &h2));
    assert!(player1_wins(&h2, &h1));
}

#[test] 
fn test_two_pair() {
    let (h1, h2) = parse_line_of_hands("2D 4H 5D QD QC 6H 6D 9H QH QS");
    if let Some(c) = h1.has_high_card() {
        assert!(c.value == Value::Five);
        assert!(c.suit == Suit::Diamonds);
    } else {
        panic!();
    }

    if let Some(c) = h1.has_pair() {
        assert!(c.value == Value::Queen);
    } else {
        panic!();
    }

    if let Some((small, big)) = h2.has_two_pair() {
        assert!(big.value == Value::Queen);
        assert!(small.value == Value::Six);
    } else {
        panic!();
    }

    assert!(!player1_wins(&h1, &h2));
    assert!(player1_wins(&h2, &h1));
}

#[test] 
fn test_two_pair_against_nothing() {
    let (h1, h2) = parse_line_of_hands("2D 4H 5D TD 3C 6H 6D 9H QH QS");
    if let Some(c) = h1.has_high_card() {
        assert!(c.value == Value::Ten);
        assert!(c.suit == Suit::Diamonds);
    } else {
        panic!();
    }

    if let Some((small, big)) = h2.has_two_pair() {
        assert!(big.value == Value::Queen);
        assert!(small.value == Value::Six);
    } else {
        panic!();
    }

    assert!(!player1_wins(&h1, &h2));
    assert!(player1_wins(&h2, &h1));
}

#[test] 
fn test_pair_of_aces_against_nothing() {
    let (h1, h2) = parse_line_of_hands("5C AD 5D AC 9C 7C 5H 8D TD KS");
    if let Some(c) = h1.has_pair() {
        assert!(c.value == Value::Ace);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_high_card() {
        assert!(c.value == Value::King);
        assert!(c.suit == Suit::Spades);
    } else {
        panic!();
    }

    assert!(player1_wins(&h1, &h2));
    assert!(!player1_wins(&h2, &h1));
}

#[test] 
fn test_nothing_hands() {
    let (h1, h2) = parse_line_of_hands("3H 7H 6S KC JS QH TD JC 2D 8S");
    if let Some(c) = h1.has_high_card() {
        assert!(c.value == Value::King);
        assert!(c.suit == Suit::Clubs);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_high_card() {
        assert!(c.value == Value::Queen);
        assert!(c.suit == Suit::Hearts);
    } else {
        panic!();
    }

    assert!(player1_wins(&h1, &h2));
    assert!(!player1_wins(&h2, &h1));
}

#[test] 
fn test_two_pairs() {
    let (h1, h2) = parse_line_of_hands("TH 8H 5C QS TC 9H 4D JC KS JS");
    if let Some(c) = h1.has_pair() {
        assert!(c.value == Value::Ten);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_pair() {
        assert!(c.value == Value::Jack);
    } else {
        panic!();
    }

    assert!(!player1_wins(&h1, &h2));
    assert!(player1_wins(&h2, &h1));
}

#[test] 
fn test_pair_over_high_card() {
    let (h1, h2) = parse_line_of_hands("7C 5H KC QH JD AS KH 4C AD 4S");
    if let Some(c) = h1.has_high_card() {
        assert!(c.value == Value::King);
        assert!(c.suit == Suit::Clubs);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_pair() {
        assert!(c.value == Value::Ace);
    } else {
        panic!();
    }

    assert!(!player1_wins(&h1, &h2));
    assert!(player1_wins(&h2, &h1));
}

#[test] 
fn test_order_of_pairs_with_higher_card() {
    let (h1, h2) = parse_line_of_hands("5H KS 9C 7D 9H 8D 3S 5D 5C AH");
    if let Some(c) = h1.has_pair() {
        assert!(c.value == Value::Nine);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_pair() {
        assert!(c.value == Value::Five);
    } else {
        panic!();
    }

    assert!(player1_wins(&h1, &h2));
    assert!(!player1_wins(&h2, &h1));
}

#[test] 
fn test_equal_pairs_high_card_wins() {
    let (h1, h2) = parse_line_of_hands("6D 7C 5D 5H 3S 5C JC 2H 5S 3D");
    if let Some(c) = h1.has_pair() {
        assert!(c.value == Value::Five);
    } else {
        panic!();
    }

    if let Some(c) = h1.has_high_card() {
        assert!(c.value == Value::Seven);
        assert!(c.suit == Suit::Clubs);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_pair() {
        assert!(c.value == Value::Five);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_high_card() {
        assert!(c.value == Value::Jack);
        assert!(c.suit == Suit::Clubs);
    } else {
        panic!();
    }

    assert!(!player1_wins(&h1, &h2));
    assert!(player1_wins(&h2, &h1));
}
#[test] 
fn test_order_of_pairs_looks_like_straight() {
    let (h1, h2) = parse_line_of_hands("JH 7C 9H 7H TC 5H 3D 6D 5D 4D");
    if let Some(c) = h1.has_pair() {
        assert!(c.value == Value::Seven);
    } else {
        panic!();
    }

    if let Some(c) = h2.has_pair() {
        assert!(c.value == Value::Five);
    } else {
        panic!();
    }

    assert!(player1_wins(&h1, &h2));
    assert!(!player1_wins(&h2, &h1));
}
