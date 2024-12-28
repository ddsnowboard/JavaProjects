use mockall::automock;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::fmt;

pub type PotAmount = i32;
pub const ANTE_AMOUNT: PotAmount = 200;
pub const BURN_COEFFICIENT: PotAmount = 2;
pub const STARTING_BANKROLL: PotAmount = 800;
pub const N_SUITS: i32 = 4;
pub const N_VALUES: i32 = 13;
pub const N_CARDS: i32 = N_SUITS * N_VALUES;

pub trait Strategy {
    fn witness(&mut self, event: PlayEvent);
    fn call_ace(&self) -> AceChoice;
    fn play(&self, opp: &Opportunity, pot_amount: PotAmount, bankroll: PotAmount) -> Response;
    fn get_name(&self) -> String;
}

#[derive(Eq, PartialEq, Clone)]
pub enum PlayEvent {
    Shuffle(Vec<Card>),
    Flip(Card),
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
pub enum Value {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Value {
    pub fn to_table_value(self) -> Option<TableValue> {
        match self {
            Value::Ace => None,
            Value::King => Some(TableValue::King),
            Value::Queen => Some(TableValue::Queen),
            Value::Jack => Some(TableValue::Jack),
            Value::Number(n) => Some(TableValue::Number(n)),
        }
    }

    pub fn to_number_ace_high(self) -> u8 {
        match self {
            Value::Number(n) => n,
            Value::Jack => 11,
            Value::Queen => 12,
            Value::King => 13,
            Value::Ace => 14,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum TableValue {
    HiAce,
    LowAce,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl PartialOrd for TableValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.to_number().cmp(&other.to_number()))
    }
}

impl Ord for TableValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl TableValue {
    pub fn to_number(self) -> u8 {
        match self {
            TableValue::LowAce => 1,
            TableValue::Number(n) => n,
            TableValue::Jack => 11,
            TableValue::Queen => 12,
            TableValue::King => 13,
            TableValue::HiAce => 14,
        }
    }
    pub fn to_value(self) -> Value {
        match self {
            TableValue::Number(n) => Value::Number(n),
            TableValue::King => Value::King,
            TableValue::Queen => Value::Queen,
            TableValue::Jack => Value::Jack,
            TableValue::HiAce | TableValue::LowAce => Value::Ace,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Card(pub Suit, pub Value);

impl Card {
    pub fn to_table_card(self) -> FlipResult {
        let Card(suit, value) = &self;
        let table_value = value.to_table_value();
        match table_value {
            Some(v) => FlipResult::Other(TableCard(*suit, v)),
            None => FlipResult::Ace(AcePicker::for_card(self)),
        }
    }
}

pub struct AcePicker {
    card: Card,
}

impl AcePicker {
    pub fn for_card(card: Card) -> Self {
        Self { card }
    }
    pub fn pick(self, choice: &AceChoice) -> TableCard {
        let Card(suit, _) = self.card;
        let value = match choice {
            AceChoice::Hi => TableValue::HiAce,
            AceChoice::Low => TableValue::LowAce,
        };
        TableCard(suit, value)
    }
}

pub enum FlipResult {
    Ace(AcePicker),
    Other(TableCard),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct TableCard(pub Suit, pub TableValue);

impl TableCard {
    pub fn to_card(self) -> Card {
        let TableCard(suit, table_value) = self;
        Card(suit, table_value.to_value())
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum AceChoice {
    Hi,
    Low,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Response {
    Pass,
    Play(PotAmount),
}

pub struct Opportunity(pub TableCard, pub TableCard);
impl Opportunity {
    pub fn swapped(&self) -> Self {
        let Self(l, r) = self;
        Self(*r, *l)
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum PlayResult {
    Inside,
    Outside,
    Double,
}

pub struct Player {
    pub money: PotAmount,
    pub strategy: Box<dyn Strategy>,
}

impl Player {
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        Self {
            money: STARTING_BANKROLL,
            strategy,
        }
    }
}

pub struct GameResult {
    pub pot: PotAmount,
    pub player_amounts: Vec<PotAmount>,
}

impl fmt::Display for GameResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Final pot: {}", self.pot)?;
        writeln!(f, "Player amounts:")?;
        for (idx, amt) in self.player_amounts.iter().enumerate() {
            writeln!(f, "{}: {}", idx, amt)?;
        }
        fmt::Result::Ok(())
    }
}

#[automock]
pub trait BetSizePolicy {
    fn get_bet_size(&self, pot_size: PotAmount, bankroll: PotAmount, ev: f64) -> PotAmount;
    fn get_name(&self) -> String;
}
