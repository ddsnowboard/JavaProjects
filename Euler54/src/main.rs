use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;


#[derive(PartialEq, Clone, Copy, Hash)]
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

#[derive(PartialEq, Clone, Copy, Ord, PartialOrd, Eq, Hash)]
enum Value {
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

#[derive(PartialEq, Clone, Copy)]
struct Card {
    suit: Suit,
    value: Value
}

#[derive(PartialEq, Clone, Copy)]
struct Hand {
    cards: [Card; 5]
}

impl Hand {
    fn new(mut cards: Vec<Card>) -> Hand {
        cards.sort_by_key(|el| el.value);
        let mut new_cards = [Card {value: Value::Ace, suit: Suit::Spades }; 5];
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
        // If they are sorted and they cover a range of 4, they are in order
        if self.cards[4].value as u32 - self.cards[0].value as u32 == 4 {
            Some(&self.cards[4])
        } else {
            None
        }
    }

    fn has_straight_flush(&self) -> Option<&Card> {
        // We know that the cards array is sorted, so we can do some stuff with that information
        if !self.have_same_suit() {
            return None;
        }

        self.has_straight()
    }

    fn find_highest(&self, suit: &Suit) -> Option<&Card> {
        for card in self.cards.iter().rev() {
            if card.suit == *suit {
                return Some(card);
            }
        }
        None
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
        let hash = self.make_counts();
        for (key, value) in hash.iter() {
            if *value == n {
                return self.contains(key);
            }
        }
        None
    }

    fn has_two_pair(&self) -> Option<(&Card, &Card)> {
        let hash = self.make_counts();
        let mut found_first = false;
        let mut first = None;
        for (key, value) in hash.iter() {
            if *value == 2 {
                if found_first {
                    let first_card = first.unwrap();
                    let second_card = self.contains(key).unwrap();
                    return Some((first_card, second_card));
                } else {
                    found_first = true;
                    first = self.contains(key);
                    continue;
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

    fn contains(&self, value: &Value) -> Option<&Card> {
        for card in self.cards.iter() {
            if card.value == *value {
                return Some(&card)
            }
        }
        None
    }

    fn has_full_house(&self) -> Option<(&Card, &Card)> {
        if let (Some(c1), Some(c2)) = (self.has_pair(), self.has_3_of_a_kind()) {
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
    let pick_winner = |p1_card: Option<&Card>, p2_card: Option<&Card> | {
        match (p1_card, p2_card) {
            (Some(_), None) => return true,
            (None, Some(_)) => return false,
            (Some(c), Some(p)) => return c.value > p.value, 
            (None, None) => return p1.cards[4].value > p2.cards[4].value
        }
    };

    if p1.has_royal_flush().is_some() || p2.has_royal_flush().is_some() {
        return p1.has_royal_flush().is_some();
    } else if p1.has_straight_flush().is_some() || p2.has_straight_flush().is_some() {
        return pick_winner(p1.has_straight_flush(), p2.has_straight_flush());
    } else if p1.has_4_of_a_kind().is_some() || p2.has_4_of_a_kind().is_some() {
        return pick_winner(p1.has_4_of_a_kind(), p2.has_4_of_a_kind());
    } else if p1.has_full_house().is_some() || p2.has_full_house().is_some() {
        return pick_winner(p1.has_full_house().map(|t| t.1), p2.has_full_house().map(|t| t.1));
    } else if p1.has_flush().is_some() || p2.has_flush().is_some() {
        return pick_winner(p1.has_flush(), p2.has_flush());
    } else if p1.has_straight().is_some() || p2.has_straight().is_some() {
        return pick_winner(p1.has_straight(), p2.has_straight());
    } else if p1.has_3_of_a_kind().is_some() || p2.has_3_of_a_kind().is_some() {
        return pick_winner(p1.has_3_of_a_kind(), p2.has_3_of_a_kind());
    } else if p1.has_two_pair().is_some() || p2.has_two_pair().is_some() {
        return pick_winner(p1.has_two_pair().map(|t| t.1), p2.has_two_pair().map(|t| t.1));
    } else if p1.has_pair().is_some() || p2.has_pair().is_some() {
        return pick_winner(p1.has_pair(), p2.has_pair());
    }
    pick_winner(None, None)
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut win_count = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            let mut cards = line.split_whitespace();
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
            if player1_wins(&h1, &h2) {
                win_count += 1;
            }
        }
    }
    println!("p1 won {} times", win_count);
}
