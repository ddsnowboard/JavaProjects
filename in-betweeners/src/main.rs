use ordered_float::OrderedFloat;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rayon::prelude::*;
use std::cmp::{min, Ord, Ordering, PartialOrd};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::stdin;
use std::sync::LazyLock;

type PotAmount = i32;
const ANTE_AMOUNT: PotAmount = 200;
const BURN_COEFFICIENT: PotAmount = 2;
const STARTING_BANKROLL: PotAmount = 800;

macro_rules! box_strategies {
    () => {(Vec::<String>::new(), || {Vec::<Box<dyn Strategy>>::new()})};
    // Handle hanging comma
    ($($ss:expr),*,) => {box_strategies!($($ss),*)};
    ($($ss:expr),*) => {{
        let name_vec = vec![
            $(stringify!($ss)),*
        ];
        fn f() -> Vec<Box<dyn Strategy>> {
            vec![
                $(Box::new($ss)),+
            ]
        }
        (name_vec, f)
    }};
}

fn main() {
    let (names, generate_strategies) = box_strategies!(
        /*
        BasicStrategy {
            bet_size_policy: BiggestBet {}
        },
        BasicStrategy {
            bet_size_policy: ConstantBet::new(200)
        },
        BasicStrategy {
            bet_size_policy: PoorMansKelly {}
        },
        OptimalStrategy::new(ConstantBet::new(10)),
        OptimalStrategy::new(ConstantBet::new(50)),
        OptimalStrategy::new(ConstantBet::new(200)),
        OptimalStrategy::new(ConstantBet::new(500)),
        OptimalStrategy::new(ConstantBet::new(800)),
        OptimalStrategy::new(BiggestBet {}),
        OptimalStrategy::new(PoorMansKelly {}),

        OptimalStrategyConstantAceChoice::new(AceChoice::Hi, PoorMansKelly {}),
        OptimalStrategyConstantAceChoice::new(AceChoice::Low, PoorMansKelly {}),
        */
        MiddleOutside::with_values(Value::Number(3), Value::Number(10)),
        MiddleOutside::with_values(Value::Number(3), Value::Jack),
        MiddleOutside::with_values(Value::Number(3), Value::Queen),
        MiddleOutside::with_values(Value::Number(3), Value::King),

        MiddleOutside::with_values(Value::Number(4), Value::Number(10)),
        MiddleOutside::with_values(Value::Number(4), Value::Jack),
        MiddleOutside::with_values(Value::Number(4), Value::Queen),
        MiddleOutside::with_values(Value::Number(4), Value::King),

        MiddleOutside::with_values(Value::Number(5), Value::Jack),
        MiddleOutside::with_values(Value::Number(5), Value::Queen),
        MiddleOutside::with_values(Value::Number(5), Value::King),
        BasicStrategy {
            bet_size_policy: ConstantBet::new(200)
        },
    );
    let results: Vec<_> = (0..100000)
        .into_par_iter()
        .map(|_idx| {
            let mut g = Game::new(generate_strategies());
            g.play()
        })
        .collect();
    let mut amounts_by_player: HashMap<usize, Vec<PotAmount>> = HashMap::new();
    results.into_iter().for_each(|r| {
        r.player_amounts
            .iter()
            .enumerate()
            .for_each(|(idx, amount)| {
                amounts_by_player.entry(idx).or_insert(vec![]).push(*amount);
            });
    });
    let mut averages: Vec<_> = amounts_by_player
        .into_iter()
        .map(|(idx, v)| (idx, (v.iter().sum::<i32>() as f64) / (v.len() as f64)))
        .collect();
    averages.sort_by_key(|(idx, _)| *idx);
    let averages: Vec<_> = averages
        .into_iter()
        .map(|(_, final_amount)| final_amount)
        .zip(names)
        .map(|(final_amount, name)| (name, final_amount))
        .collect();
    for (name, final_amount) in averages.into_iter() {
        println!("{} ended with {}", name, final_amount);
    }
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
impl Opportunity {
    fn swapped(&self) -> Self {
        let Self(l, r) = self;
        Self(*r, *l)
    }
}
enum Response {
    Pass,
    Play(PotAmount),
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
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

struct GameResult {
    pot: PotAmount,
    player_amounts: Vec<PotAmount>,
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
                    assert!(amount > 0, "Amount must be positive");
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
        }
        self.discards.push(left_card.to_card());
        self.discards.push(right_card.to_card());
    }

    fn play(&mut self) -> GameResult {
        let player_indices_forever = (0..self.players.len()).cycle();
        for idx in player_indices_forever {
            // If everyone is broke or the game is over
            if self.players.iter().filter(|p| p.money > 0).next().is_none() || self.current_pot == 0
            {
                break;
            }
            let player = &self.players[idx];
            if player.money <= 0 {
                continue;
            } else {
                self.play_once(idx);
            }
        }
        self.to_result()
    }

    fn to_result(&self) -> GameResult {
        GameResult {
            pot: self.current_pot,
            player_amounts: self.players.iter().map(|p| p.money).collect(),
        }
    }
}

fn calculate_ev(left_card: &TableCard, right_card: &TableCard, potential_cards: &[Card]) -> f64 {
    potential_cards
        .iter()
        .map(|c| get_result(left_card, &c, right_card))
        .map(|r| match r {
            PlayResult::Inside => 1,
            PlayResult::Outside => -1,
            PlayResult::Double => -2,
        })
        .sum::<i32>() as f64
        / potential_cards.len() as f64
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
                assert!(amount > 0, "You must bet a positive amount");
                return Response::Play(amount);
            }
        }
    }
}

trait BetSizePolicy {
    fn get_bet_size(&self, pot_size: PotAmount, bankroll: PotAmount, ev: f64) -> PotAmount;
}

struct BiggestBet {}

impl BetSizePolicy for BiggestBet {
    fn get_bet_size(&self, pot_size: PotAmount, bankroll: PotAmount, _ev: f64) -> PotAmount {
        min(pot_size, bankroll)
    }
}

struct ConstantBet {
    amount: PotAmount,
}

impl ConstantBet {
    fn new(amount: PotAmount) -> Self {
        Self { amount }
    }
}

impl BetSizePolicy for ConstantBet {
    fn get_bet_size(&self, pot_size: PotAmount, bankroll: PotAmount, _ev: f64) -> PotAmount {
        min(self.amount, min(pot_size, bankroll))
    }
}

struct PoorMansKelly {}

impl BetSizePolicy for PoorMansKelly {
    /// I really should derive the real Kelly fraction for this situation, but I'm going to use the
    /// one that ignores double burning. What's the worst that could happen?
    fn get_bet_size(&self, pot_size: PotAmount, bankroll: PotAmount, ev: f64) -> PotAmount {
        if bankroll < 2 && ev > 0.5 {
            return 1;
        }
        // Wait is this right?
        let coefficient = ev;
        let bet = min(
            pot_size,
            min(bankroll, (bankroll as f64 * coefficient) as PotAmount),
        );
        // println!("Pot is {}, bank is {}, bet is {}", pot_size, bankroll, bet);
        bet
    }
}

struct BasicStrategy<P: BetSizePolicy> {
    bet_size_policy: P,
}

impl<P: BetSizePolicy> Strategy for BasicStrategy<P> {
    fn witness(&mut self, _event: PlayEvent) {}
    fn call_ace(&self) -> AceChoice {
        AceChoice::Low
    }
    fn play(&self, opp: &Opportunity, pot_amount: PotAmount, bankroll: PotAmount) -> Response {
        let Opportunity(left_card, right_card) = opp;
        let known_remaining_cards: Vec<_> = BASE_DECK
            .clone()
            .into_iter()
            .filter(|&c| c != left_card.to_card() && c != right_card.to_card())
            .collect();
        let ev = calculate_ev(left_card, right_card, &known_remaining_cards);
        /*
        println!(
            "Cards are {:?} and {:?} and ev is {}",
            left_card, right_card, ev
        );
            */
        if ev > 0.0 {
            let potential_bet = self.bet_size_policy.get_bet_size(pot_amount, bankroll, ev);
            if potential_bet > 0 {
                Response::Play(potential_bet)
            } else {
                Response::Pass
            }
        } else {
            Response::Pass
        }
    }
}

struct OptimalStrategy<P: BetSizePolicy> {
    remaining_cards: HashSet<Card>,
    bet_size_policy: P,
}

impl<P: BetSizePolicy> OptimalStrategy<P> {
    fn new(bet_size_policy: P) -> Self {
        Self {
            remaining_cards: BASE_DECK.iter().cloned().collect(),
            bet_size_policy,
        }
    }
}

impl<P: BetSizePolicy> Strategy for OptimalStrategy<P> {
    fn witness(&mut self, event: PlayEvent) {
        match event {
            PlayEvent::Shuffle(new_deck) => {
                self.remaining_cards = new_deck.into_iter().collect();
            }
            PlayEvent::Flip(card) => {
                self.remaining_cards.remove(&card);
            }
        }
    }
    fn call_ace(&self) -> AceChoice {
        let options = vec![
            // The suit doesn't matter
            (AceChoice::Low, TableCard(Suit::Hearts, TableValue::LowAce)),
            (AceChoice::Hi, TableCard(Suit::Hearts, TableValue::HiAce)),
        ];
        options
            .into_iter()
            .max_by_key(|(_, card)| {
                self.remaining_cards
                    .iter()
                    .map(|other_card| {
                        let remaining_cards: Vec<_> = self
                            .remaining_cards
                            .iter()
                            .filter(|c| *c != other_card)
                            .cloned()
                            .collect();
                        let other_table_card = match other_card.to_table_card() {
                            FlipResult::Other(table_card) => table_card,
                            FlipResult::Ace(picker) => picker.pick(&AceChoice::Hi),
                        };
                        // If the EV is less than 0 or we can't play, we just won't
                        OrderedFloat(if is_playable(card, &other_table_card) {
                            f64::max(
                                0.0,
                                calculate_ev(&card, &other_table_card, &remaining_cards),
                            )
                        } else {
                            0.0
                        })
                    })
                    .sum::<OrderedFloat<_>>()
            })
            .unwrap()
            .0
    }
    fn play(&self, opp: &Opportunity, pot_amount: PotAmount, bankroll: PotAmount) -> Response {
        let Opportunity(left_card, right_card) = opp;
        let ev = calculate_ev(
            left_card,
            right_card,
            &self.remaining_cards.iter().cloned().collect::<Vec<_>>(),
        );
        /*
        println!(
            "Cards are {:?} and {:?} and ev is {}",
            left_card, right_card, ev
        );
            */
        if ev > 0.0 {
            let potential_bet = self.bet_size_policy.get_bet_size(pot_amount, bankroll, ev);
            if potential_bet > 0 {
                Response::Play(potential_bet)
            } else {
                Response::Pass
            }
        } else {
            Response::Pass
        }
    }
}

struct OptimalStrategyConstantAceChoice<P: BetSizePolicy> {
    ace_choice: AceChoice,
    underlying_strategy: OptimalStrategy<P>,
}

impl<P: BetSizePolicy> OptimalStrategyConstantAceChoice<P> {
    fn new(ace_choice: AceChoice, bet_size_policy: P) -> Self {
        Self {
            ace_choice,
            underlying_strategy: OptimalStrategy::new(bet_size_policy),
        }
    }
}

impl<P: BetSizePolicy> Strategy for OptimalStrategyConstantAceChoice<P> {
    fn witness(&mut self, event: PlayEvent) {
        self.underlying_strategy.witness(event)
    }
    fn call_ace(&self) -> AceChoice {
        self.ace_choice.clone()
    }
    fn play(&self, opp: &Opportunity, pot_amount: PotAmount, bankroll: PotAmount) -> Response {
        self.underlying_strategy.play(opp, pot_amount, bankroll)
    }
}

struct MiddleOutside<U: Strategy> {
    low_value: Value,
    high_value: Value,
    count: i32,
    underlying: U,
    bet_size_policy: ConstantBet,
}

impl MiddleOutside<BasicStrategy<ConstantBet>> {
    fn new() -> Self {
        let default_low = Value::Number(4);
        let default_high = Value::Number(10);
        Self::with_values(default_low, default_high)
    }

    fn with_values(low: Value, high: Value) -> Self {
        Self::with_values_and_underlying(
            low,
            high,
            BasicStrategy {
                bet_size_policy: ConstantBet::new(100),
            },
        )
    }
}

impl<U: Strategy> MiddleOutside<U> {
    fn with_values_and_underlying(low: Value, high: Value, underlying: U) -> Self {
        if low.to_number_ace_high() > high.to_number_ace_high() {
            Self::with_values_and_underlying(high, low, underlying)
        } else {
            Self {
                low_value: low,
                high_value: high,
                count: 0,
                underlying,
                bet_size_policy: ConstantBet::new(100),
            }
        }
    }

    fn is_outside(&self, card: &Card) -> bool {
        let Card(_, value) = card;
        let value_n = value.to_number_ace_high();
        value_n >= self.high_value.to_number_ace_high()
            || value_n <= self.low_value.to_number_ace_high()
    }
}

impl<U: Strategy> Strategy for MiddleOutside<U> {
    fn witness(&mut self, event: PlayEvent) {
        match event {
            PlayEvent::Shuffle(_) => {
                self.count = 0;
            }
            PlayEvent::Flip(c) => {
                if self.is_outside(&c) {
                    self.count += 1;
                } else {
                    self.count -= 1;
                }
            }
        }
    }
    fn call_ace(&self) -> AceChoice {
        // Maybe we'll come back to this
        AceChoice::Low
    }
    fn play(&self, opp: &Opportunity, pot_amount: PotAmount, bankroll: PotAmount) -> Response {
        let Opportunity(left_card, right_card) = opp;
        let TableCard(_, left_value) = left_card;
        let TableCard(_, right_value) = right_card;
        if left_value > right_value {
            return self.play(&opp.swapped(), pot_amount, bankroll);
        }
        assert!(
            left_value <= right_value,
            "Left_card must be lower than right_card by now"
        );
        if self.is_outside(&left_card.clone().to_card())
            && self.is_outside(&right_card.clone().to_card())
        {
            let n_burns = if *left_value == TableValue::LowAce && *right_value == TableValue::HiAce
            {
                2
            } else {
                6 // This assumes that the other 3 examples of each card are still in the deck
            };
            let n_wins = (right_value.to_number() - left_value.to_number() - 1) as i32;
            let n_values = 13;
            let n_losses = (n_values - n_wins - n_burns) as i32;
            let ev_kinda = (n_wins + self.count - 2 * n_burns - n_losses) as f64
                / (n_burns + n_losses + n_wins) as f64;
            if ev_kinda > 0.0 {
                Response::Play(
                    self.bet_size_policy
                        .get_bet_size(pot_amount, bankroll, ev_kinda),
                )
            } else {
                Response::Pass
            }
        } else {
            self.underlying.play(opp, pot_amount, bankroll)
        }
    }
}
