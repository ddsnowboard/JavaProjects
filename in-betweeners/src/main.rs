use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::{min, Ord, Ordering, PartialOrd};
use std::io::stdin;
use std::sync::LazyLock;

type PotAmount = i32;
const ANTE_AMOUNT: PotAmount = 200;
const BURN_COEFFICIENT: PotAmount = 2;
const STARTING_BANKROLL: PotAmount = 800;

fn main() {
    let mut g = Game::new(vec![
        Box::new(BasicStrategy {}),
        Box::new(BasicStrategy {}),
        Box::new(ManualStrategy {}),
    ]);
    g.play()
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

    fn to_number_ace_high(self) -> u8 {
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
enum TableValue {
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
    fn to_number(self) -> u8 {
        match self {
            TableValue::LowAce => 1,
            TableValue::Number(n) => n,
            TableValue::Jack => 11,
            TableValue::Queen => 12,
            TableValue::King => 13,
            TableValue::HiAce => 14,
        }
    }
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
}

trait Strategy {
    fn witness(&mut self, event: PlayEvent);
    fn call_ace(&self) -> AceChoice;
    fn play(&self, opp: &Opportunity, pot_amount: PotAmount, bankroll: PotAmount) -> Response;
}

struct Player {
    money: PotAmount,
    strategy: Box<dyn Strategy>,
}

impl Player {
    fn new(strategy: Box<dyn Strategy>) -> Self {
        Self {
            money: STARTING_BANKROLL,
            strategy,
        }
    }
}

enum PlayResult {
    Inside,
    Outside,
    Double,
}

fn is_playable(left_card: &TableCard, right_card: &TableCard) -> bool {
    let TableCard(_, left_value) = left_card;
    let TableCard(_, right_value) = right_card;
    left_value != right_value
}

fn get_result(left_card: &TableCard, middle_card: &Card, right_card: &TableCard) -> PlayResult {
    /// This assumes that `low_side` is less than `high_side`
    fn between_ascending(
        low_value: &TableValue,
        middle_value: &Value,
        high_value: &TableValue,
    ) -> PlayResult {
        let low_n = low_value.to_number();
        let middle_n = middle_value.to_number_ace_high();
        let high_n = high_value.to_number();
        if *middle_value == low_value.to_value() || *middle_value == high_value.to_value() {
            PlayResult::Double
        } else if middle_n > low_n && middle_n < high_n {
            PlayResult::Inside
        } else {
            PlayResult::Outside
        }
    }
    assert!(
        is_playable(&left_card, &right_card),
        "It's impossible to hit on this"
    );
    let TableCard(_, left_value) = left_card;
    let Card(_, middle_value) = middle_card;
    let TableCard(_, right_value) = right_card;
    if left_value < right_value {
        between_ascending(left_value, middle_value, right_value)
    } else {
        between_ascending(right_value, middle_value, left_value)
    }
}

struct Game {
    discards: Vec<Card>,
    remaining_deck: Vec<Card>,
    current_pot: PotAmount,
    players: Vec<Player>,
}

impl Game {
    fn new(strategies: Vec<Box<dyn Strategy>>) -> Self {
        let mut rng = thread_rng();
        let mut out = Self {
            discards: vec![],
            remaining_deck: BASE_DECK.clone(),
            current_pot: ANTE_AMOUNT * (strategies.len() as PotAmount),
            players: strategies.into_iter().map(Player::new).collect(),
        };
        out.remaining_deck.shuffle(&mut rng);
        out
    }

    fn witness(&mut self, event: PlayEvent) {
        self.players
            .iter_mut()
            .for_each(|p| p.strategy.witness(event.clone()))
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();

        assert!(self.remaining_deck.is_empty());
        let mut new_deck: Vec<Card> = self.discards.drain(..).collect();
        self.witness(PlayEvent::Shuffle(new_deck.clone()));
        new_deck.shuffle(&mut rng);
        self.remaining_deck.append(&mut new_deck);
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
            FlipResult::Ace(picker) => picker.pick(&self.players[player_idx].strategy.call_ace()),
        };
        let right_card = match self.draw_or_shuffle().to_table_card() {
            FlipResult::Other(tc) => tc,
            FlipResult::Ace(picker) => picker.pick(&AceChoice::Hi),
        };

        if is_playable(&left_card, &right_card) {
            let player = &mut self.players[player_idx];
            match player.strategy.play(
                &Opportunity(left_card, right_card),
                self.current_pot,
                player.money,
            ) {
                Response::Pass => {
                    // Do nothing
                }
                Response::Play(amount) => {
                    assert!(amount <= self.current_pot, "Amount must be less than pot");
                    assert!(
                        amount <= player.money,
                        "Amount must be less than your bankroll"
                    );
                    let middle_card = self.draw_or_shuffle();
                    let player = &mut self.players[player_idx];
                    let amount_to_give_player =
                        match get_result(&left_card, &middle_card, &right_card) {
                            PlayResult::Inside => amount,
                            PlayResult::Outside => -amount,
                            PlayResult::Double => -BURN_COEFFICIENT * amount,
                        };
                    self.current_pot -= amount_to_give_player;
                    player.money += amount_to_give_player;
                    self.discards.push(middle_card);
                }
            }
            self.discards.push(left_card.to_card());
            self.discards.push(right_card.to_card());
        }
    }

    fn play(&mut self) {
        let player_indices_forever = (0..self.players.len()).cycle();
        for idx in player_indices_forever {
            if self.current_pot == 0 {
                return;
            }
            let player = &self.players[idx];
            if player.money == 0 {
                continue;
            } else {
                self.play_once(idx);
            }
        }
    }
}

struct ManualStrategy {}
impl Strategy for ManualStrategy {
    fn witness(&mut self, event: PlayEvent) {
        match event {
            PlayEvent::Shuffle(_) => println!("Shuffled"),
            PlayEvent::Flip(c) => println!("Flipped {:?}", c),
        }
    }
    fn call_ace(&self) -> AceChoice {
        loop {
            println!("Ace [h]igh or [l]ow?");
            let mut i = String::new();
            stdin().read_line(&mut i).unwrap();
            match i.as_bytes()[0] {
                b'h' => return AceChoice::Hi,
                b'l' => return AceChoice::Low,
                _ => {}
            }
        }
    }
    fn play(
        &self,
        Opportunity(left_card, right_card): &Opportunity,
        pot_amount: PotAmount,
        bankroll: PotAmount,
    ) -> Response {
        loop {
            println!("Pot is {}; you have {}", pot_amount, bankroll);
            println!("{:?}     {:?}", left_card, right_card);
            println!("Bet a number or just press Enter to pass");
            let mut i = String::new();
            stdin().read_line(&mut i).unwrap();
            let i = i.trim();
            if i.is_empty() {
                return Response::Pass;
            } else if let Ok(amount) = i.parse() {
                return Response::Play(amount);
            }
        }
    }
}

struct BasicStrategy {}

impl Strategy for BasicStrategy {
    fn witness(&mut self, _event: PlayEvent) {}
    fn call_ace(&self) -> AceChoice {
        AceChoice::Low
    }
    fn play(&self, opp: &Opportunity, pot_amount: PotAmount, bankroll: PotAmount) -> Response {
        let Opportunity(left_card, right_card) = opp;
        let known_remaining_cards = BASE_DECK
            .clone()
            .into_iter()
            .filter(|&c| c != left_card.to_card() && c != right_card.to_card());
        let ev: i32 = known_remaining_cards
            .map(|c| get_result(left_card, &c, right_card))
            .map(|r| match r {
                PlayResult::Inside => 1,
                PlayResult::Outside => -1,
                PlayResult::Double => -2,
            })
            .sum();
        if ev > 0 {
            Response::Play(min(50, min(pot_amount, bankroll)))
        } else {
            Response::Pass
        }
    }
}
