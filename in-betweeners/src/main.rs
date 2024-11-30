use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::sync::LazyLock;

type PotAmount = u32;
const ANTE_AMOUNT: PotAmount = 200;
const BURN_COEFFICIENT: PotAmount = 2;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Value {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Value {
    fn to_table_value(self) -> Option<TableValue> {
        match self {
            Value::Ace => None,
            Value::King => Some(TableValue::King),
            Value::Queen => Some(TableValue::Queen),
            Value::Jack => Some(TableValue::Jack),
            Value::Number(n) => Some(TableValue::Number(n)),
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum TableValue {
    HiAce,
    LowAce,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl PartialOrd<Value> for TableValue {
    fn partial_cmp(&self, other: &Value) -> Option<Ordering> {
        // hmm this is weird because all aces are equal, I think, but a low ace is less than a 2
        // and a high ace is greater than a 2. Mysterious.
        // Also HiAce == Ace and Ace == LowAce, but HiAce != LowAce. Does that mean I can't
        // implement PartialEq (and therefore can't implement PartialOrd) for these? That's
        // troubling.
        let my_value: u8 = match self {
            TableValue::LowAce => 1,
            TableValue::Number(n) => *n,
            TableValue::Jack => 11,
            TableValue::Queen => 12,
            TableValue::King => 13,
            TableValue::HiAce => 14,
        };
        let their_value: u8 = match other {
            Value::Number(n) => *n,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
            // Third ace is always high, I guess
            Value::Ace => 14,
        };
        // All aces are equal
        if self.to_value() == Value::Ace && *other == Value::Ace {
            Some(Ordering::Equal)
        } else {
            Some(my_value.cmp(&their_value))
        }
    }
}

impl TableValue {
    fn to_value(self) -> Value {
        match self {
            TableValue::Number(n) => Value::Number(n),
            TableValue::King => Value::King,
            TableValue::Queen => Value::Queen,
            TableValue::Jack => Value::Jack,
            TableValue::HiAce | TableValue::LowAce => Value::Ace,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Card(Suit, Value);

impl Card {
    fn to_table_card(self) -> FlipResult {
        let Card(suit, value) = &self;
        let table_value = value.to_table_value();
        match table_value {
            Some(v) => FlipResult::Other(TableCard(*suit, v)),
            None => FlipResult::Ace(AcePicker::for_card(self)),
        }
    }
}

struct AcePicker {
    card: Card,
}

impl AcePicker {
    fn for_card(card: Card) -> Self {
        Self { card }
    }
    fn pick(self, choice: &AceChoice) -> TableCard {
        let Card(suit, _) = self.card;
        let value = match choice {
            AceChoice::Hi => TableValue::HiAce,
            AceChoice::Low => TableValue::LowAce,
        };
        TableCard(suit, value)
    }
}

enum FlipResult {
    Ace(AcePicker),
    Other(TableCard),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct TableCard(Suit, TableValue);

impl TableCard {
    fn to_card(self) -> Card {
        let TableCard(suit, table_value) = self;
        Card(suit, table_value.to_value())
    }
}

static BASE_DECK: LazyLock<Vec<Card>> = LazyLock::new(|| {
    let all_numbers = (2..=10).map(Value::Number);
    let all_values = all_numbers.chain(vec![Value::Ace, Value::King, Value::Queen, Value::Jack]);
    let all_suits = [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs];
    all_values
        .flat_map(|v| all_suits.iter().copied().map(move |s| Card(s, v)))
        .collect::<Vec<_>>()
});

struct Opportunity(TableCard, TableCard);
enum Response {
    Pass,
    Play(PotAmount),
}

enum AceChoice {
    Hi,
    Low,
}

#[derive(Eq, PartialEq, Clone)]
enum PlayEvent {
    Shuffle(Vec<Card>),
    Flip(Card),
    PotResize(PotAmount),
}

trait Strategy {
    fn witness(&mut self, event: PlayEvent);
    fn call_ace(&self) -> AceChoice;
    fn play(&self, opp: &Opportunity) -> Response;
}

struct Game {
    discards: Vec<TableCard>,
    remaining_deck: Vec<Card>,
    current_pot: PotAmount,
    players: Vec<Box<dyn Strategy>>,
}

enum PlayResult {
    Inside,
    Outside,
    Double,
}

fn get_result(left_card: TableCard, middle_card: TableCard, right_card: TableCard) -> PlayResult {
    let TableCard(_, left_value) = left_card;
    let Card(_, middle_value) = middle_card;
    let TableCard(_, right_value) = right_card;
    if (middle_value > left_value && middle_value < right_value)
        || (middle_value < left_value && middle_value > right_value)
    {
        PlayResult::Inside
    } else if middle_value == left_value || middle_value == right_value {
        PlayResult::Double
    } else {
        PlayResult::Outside
    }
}

impl Game {
    fn new(players: Vec<Box<dyn Strategy>>) -> Self {
        Self {
            discards: vec![],
            remaining_deck: BASE_DECK.clone(),
            current_pot: ANTE_AMOUNT * (players.len() as PotAmount),
            players,
        }
    }

    fn witness(&mut self, event: PlayEvent) {
        self.players
            .iter_mut()
            .for_each(|p| p.witness(event.clone()))
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();

        assert!(self.remaining_deck.is_empty());
        let mut new_deck: Vec<Card> = self.discards.drain(..).map(|tc| tc.to_card()).collect();
        self.witness(PlayEvent::Shuffle(new_deck.clone()));
        new_deck.shuffle(&mut rng);
    }

    fn draw_or_shuffle(&mut self) -> Card {
        let c = self.remaining_deck.pop().unwrap_or_else(|| {
            self.shuffle();
            self.remaining_deck
                .pop()
                .expect("We just shuffled but there are no cards")
        });
        self.witness(PlayEvent::Flip(c));
        c
    }

    fn play_once(&mut self, player_idx: usize) {
        let left_card: Card = self.draw_or_shuffle();
        let left_card: TableCard = match left_card.to_table_card() {
            FlipResult::Other(tc) => tc,
            FlipResult::Ace(picker) => picker.pick(&self.players[player_idx].call_ace()),
        };
        let right_card = match self.draw_or_shuffle().to_table_card() {
            FlipResult::Other(tc) => tc,
            FlipResult::Ace(picker) => picker.pick(&AceChoice::Hi),
        };

        match self.players[player_idx].play(&Opportunity(left_card, right_card)) {
            Response::Pass => {
                self.discards.push(left_card);
                self.discards.push(right_card);
            }
            Response::Play(amount) => {
                let middle_card = self.draw_or_shuffle();
                match get_result(left_card, middle_card, right_card) {}
            }
        }
    }
}
