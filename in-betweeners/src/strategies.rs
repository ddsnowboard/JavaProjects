use crate::bet_size_policies::*;
use crate::models::*;
use crate::utils::*;
use ordered_float::OrderedFloat;
use std::collections::HashSet;
use std::io::stdin;

pub struct ManualStrategy {}
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

    fn get_name(&self) -> String {
        String::from("ManualStrategy")
    }
}

pub struct BasicStrategy<P: BetSizePolicy> {
    pub bet_size_policy: P,
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
        if ev > 0.0 {
            let potential_bet = self.bet_size_policy.get_bet_size(pot_amount, bankroll, ev);
            if let Some(bet) = potential_bet.filter(|bet| *bet > 0) {
                Response::Play(bet)
            } else {
                Response::Pass
            }
        } else {
            Response::Pass
        }
    }

    fn get_name(&self) -> String {
        format!(
            "BasicStrategy<bet policy={}>",
            self.bet_size_policy.get_name()
        )
    }
}

#[cfg(test)]
mod basic_strategy_test {
    use crate::models::*;
    use crate::strategies::test_common;
    use crate::strategies::*;

    #[test]
    fn basic_strategy_calls_bet_policy_on_good_opporunity() {
        let mut s = BasicStrategy {
            bet_size_policy: MockBetSizePolicy::new(),
        };
        let play_amount = 333;
        s.bet_size_policy
            .expect_get_bet_size()
            .times(1)
            .return_const(play_amount);
        assert_eq!(
            test_common::good_opportunity(&mut s),
            Response::Play(play_amount)
        );
    }

    #[test]
    fn basic_strategy_does_not_call_bet_policy_on_bad_opporunity() {
        let mut s = BasicStrategy {
            bet_size_policy: MockBetSizePolicy::new(),
        };
        let bad_opportunity = Opportunity(
            TableCard(Suit::Hearts, TableValue::King),
            TableCard(Suit::Hearts, TableValue::HiAce),
        );
        let play_amount = 333;
        s.bet_size_policy
            .expect_get_bet_size()
            .times(0)
            .return_const(play_amount);
        let pot_amount = 500;
        let bankroll = 11000;
        let response = s.play(&bad_opportunity, pot_amount, bankroll);
        assert_eq!(response, Response::Pass);
    }
}

pub struct OptimalStrategy<P: BetSizePolicy> {
    remaining_cards: HashSet<Card>,
    bet_size_policy: P,
}

impl<P: BetSizePolicy> OptimalStrategy<P> {
    pub fn new(bet_size_policy: P) -> Self {
        Self {
            remaining_cards: BASE_DECK.iter().cloned().collect(),
            bet_size_policy,
        }
    }

    fn get_remaining_cards(&self) -> Vec<Card> {
        let it: Box<dyn Iterator<Item = &Card>> = if !self.remaining_cards.is_empty() {
            Box::new(self.remaining_cards.iter())
        } else {
            Box::new(BASE_DECK.iter())
        };
        it.cloned().collect()
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
        let remaining_cards = self.get_remaining_cards();
        options
            .into_iter()
            .max_by_key(|(_, card)| {
                remaining_cards
                    .iter()
                    .map(|other_card| {
                        let remaining_cards: Vec<_> = {
                            let cards_after_next_flip: Vec<_> = self
                                .remaining_cards
                                .iter()
                                .filter(|c| *c != other_card)
                                .cloned()
                                .collect();
                            if cards_after_next_flip.is_empty() {
                                // Our card will be after the shuffle
                                BASE_DECK.iter().cloned().collect()
                            } else {
                                cards_after_next_flip
                            }
                        };
                        let other_table_card = match other_card.to_table_card() {
                            FlipResult::Other(table_card) => table_card,
                            // If there's another ace, it will always be high
                            FlipResult::Ace(picker) => picker.pick(&AceChoice::Hi),
                        };
                        // If the EV is less than 0 or we can't play, we just won't
                        OrderedFloat(if is_playable(card, &other_table_card) {
                            f64::max(0.0, calculate_ev(card, &other_table_card, &remaining_cards))
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
        let ev = calculate_ev(left_card, right_card, &self.get_remaining_cards());
        if ev > 0.0 {
            let potential_bet = self.bet_size_policy.get_bet_size(pot_amount, bankroll, ev);
            if let Some(bet) = potential_bet.filter(|bet| *bet > 0) {
                Response::Play(bet)
            } else {
                Response::Pass
            }
        } else {
            Response::Pass
        }
    }

    fn get_name(&self) -> String {
        format!(
            "OptimalStrategy<bet policy={}>",
            self.bet_size_policy.get_name()
        )
    }
}

#[cfg(test)]
mod optimal_strategy_test {
    use crate::models::*;
    use crate::strategies::test_common;
    use crate::strategies::*;
    use crate::utils::BASE_DECK;

    #[test]
    fn optimal_strategy_plays_on_good_opporunity_with_full_deck() {
        let bet_size = 20;
        let mut s = OptimalStrategy::new(ConstantBet::new(bet_size));
        assert_eq!(
            test_common::good_opportunity(&mut s),
            Response::Play(bet_size)
        );
    }

    #[test]
    fn optimal_strategy_does_not_play_on_bad_opporunity_with_full_deck() {
        let mut s = OptimalStrategy::new(ConstantBet::new(20));
        s.witness(PlayEvent::Shuffle(BASE_DECK.clone()));
        let bad_opportunity = Opportunity(
            TableCard(Suit::Hearts, TableValue::King),
            TableCard(Suit::Hearts, TableValue::HiAce),
        );
        let pot_amount = 500;
        let bankroll = 11000;
        let response = s.play(&bad_opportunity, pot_amount, bankroll);
        assert_eq!(response, Response::Pass);
    }

    #[test]
    fn optimal_strategy_plays_bad_opportunity_when_only_good_cards_remain() {
        let bet_size = 20;
        let mut s = OptimalStrategy::new(ConstantBet::new(bet_size));
        s.witness(PlayEvent::Shuffle(BASE_DECK.clone()));
        let bad_opportunity = Opportunity(
            TableCard(Suit::Hearts, TableValue::Number(9)),
            TableCard(Suit::Hearts, TableValue::HiAce),
        );
        {
            let mut w = |c: &Card| {
                s.witness(PlayEvent::Flip(*c));
            };
            BASE_DECK.iter().for_each(|c| {
                let Card(_, value) = c;
                match value {
                    Value::Ace => w(c),
                    Value::Number(n) if *n <= 9 => w(c),
                    _ => {}
                }
            });
        }
        let pot_amount = 500;
        let bankroll = 11000;
        let response = s.play(&bad_opportunity, pot_amount, bankroll);
        assert_eq!(response, Response::Play(bet_size));
    }

    #[test]
    fn optimal_strategy_works_if_it_got_the_last_card() {
        // If it got the last card, then it'll know that there's no card left. Will it panic? It
        // shouldn't.
        let bet_size = 20;
        let mut s = OptimalStrategy::new(ConstantBet::new(bet_size));
        s.witness(PlayEvent::Shuffle(BASE_DECK.clone()));
        BASE_DECK
            .iter()
            .for_each(|c| s.witness(PlayEvent::Flip(*c)));
        // It doesn't matter what this is
        let opportunity = Opportunity(
            TableCard(Suit::Hearts, TableValue::Number(9)),
            TableCard(Suit::Hearts, TableValue::HiAce),
        );
        let pot_amount = 500;
        let bankroll = 11000;
        assert_eq!(s.call_ace(), AceChoice::Low);
        let response = s.play(&opportunity, pot_amount, bankroll);
        assert_eq!(response, Response::Pass);
    }

    #[test]
    fn optimal_strategy_works_when_only_one_card_left() {
        // If it got the last card, then it'll know that there's no card left. Will it panic? It
        // shouldn't.
        let bet_size = 20;
        let mut s = OptimalStrategy::new(ConstantBet::new(bet_size));
        s.witness(PlayEvent::Shuffle(BASE_DECK.clone()));
        let deck_with_aces_first: Vec<_> = {
            let mut deck: Vec<_> = BASE_DECK.iter().cloned().collect();
            deck.sort_by_key(|Card(_, value)| if *value == Value::Ace { 0 } else { 1 });
            deck
        };
        deck_with_aces_first
            .into_iter()
            // There must be 1 card left
            .skip(1)
            .for_each(|c| s.witness(PlayEvent::Flip(c)));
        // It doesn't matter what this is
        let opportunity = Opportunity(
            TableCard(Suit::Hearts, TableValue::Number(9)),
            TableCard(Suit::Hearts, TableValue::HiAce),
        );
        let pot_amount = 500;
        let bankroll = 11000;
        assert_eq!(s.call_ace(), AceChoice::Low);
        let response = s.play(&opportunity, pot_amount, bankroll);
        assert_eq!(response, Response::Pass);
    }
}

pub struct OptimalStrategyConstantAceChoice<P: BetSizePolicy> {
    ace_choice: AceChoice,
    underlying_strategy: OptimalStrategy<P>,
}

impl<P: BetSizePolicy> OptimalStrategyConstantAceChoice<P> {
    pub fn new(ace_choice: AceChoice, bet_size_policy: P) -> Self {
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
        self.ace_choice
    }
    fn play(&self, opp: &Opportunity, pot_amount: PotAmount, bankroll: PotAmount) -> Response {
        self.underlying_strategy.play(opp, pot_amount, bankroll)
    }
    fn get_name(&self) -> String {
        format!(
            "OptimalStrategyConstantAceChoice<bet policy={}>",
            self.underlying_strategy.bet_size_policy.get_name()
        )
    }
}

pub struct MiddleOutside<U: Strategy> {
    pub low_value: Value,
    pub high_value: Value,
    pub count: i32,
    pub underlying: U,
    pub bet_size_policy: ConstantBet,
}

impl Default for MiddleOutside<BasicStrategy<ConstantBet>> {
    fn default() -> Self {
        Self::new()
    }
}

impl MiddleOutside<BasicStrategy<ConstantBet>> {
    pub fn new() -> Self {
        let default_low = Value::Number(4);
        let default_high = Value::Number(10);
        Self::with_values(default_low, default_high)
    }

    pub fn with_values(low: Value, high: Value) -> Self {
        Self::with_values_and_underlying(
            low,
            high,
            BasicStrategy {
                bet_size_policy: ConstantBet::new(Self::BET_SIZE),
            },
        )
    }
}

impl<U: Strategy> MiddleOutside<U> {
    const N_SUITS: u32 = 4;
    const BET_SIZE: i32 = 200;
    pub fn with_values_and_underlying(low: Value, high: Value, underlying: U) -> Self {
        if low.to_number_ace_high() > high.to_number_ace_high() {
            Self::with_values_and_underlying(high, low, underlying)
        } else {
            Self {
                low_value: low,
                high_value: high,
                count: 0,
                underlying,
                bet_size_policy: ConstantBet::new(Self::BET_SIZE),
            }
        }
    }

    pub fn is_outside(&self, card: &Card) -> bool {
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
        if self.is_outside(&(*left_card).to_card()) && self.is_outside(&(*right_card).to_card()) {
            let n_burns = if *left_value == TableValue::LowAce && *right_value == TableValue::HiAce
            {
                2
            } else {
                6 // This assumes that the other 3 examples of each card are still in the deck
            };
            let n_wins = (right_value.to_number() - left_value.to_number() - 1) as i32 * N_SUITS;
            let n_losses = (N_VALUES * N_SUITS) - n_wins - n_burns;
            let ev_kinda = (n_wins + self.count - 2 * n_burns - n_losses) as f64
                / (n_burns + n_losses + n_wins) as f64;
            if ev_kinda > 0.0 {
                if let Some(bet_size) = self
                    .bet_size_policy
                    .get_bet_size(pot_amount, bankroll, ev_kinda)
                {
                    Response::Play(bet_size)
                } else {
                    Response::Pass
                }
            } else {
                Response::Pass
            }
        } else {
            self.underlying.play(opp, pot_amount, bankroll)
        }
    }

    fn get_name(&self) -> String {
        format!(
            "MiddleOutside<bet policy={}>",
            self.bet_size_policy.get_name()
        )
    }
}

#[cfg(test)]
mod middle_outside_test {
    use crate::models::*;
    use crate::strategies::test_common;
    use crate::strategies::*;

    const BET_SIZE: PotAmount = 200;

    #[test]
    fn middle_outside_calls_good_play_with_full_deck() {
        let mut s = MiddleOutside::new();
        assert_eq!(
            test_common::good_opportunity(&mut s),
            Response::Play(BET_SIZE)
        );
    }

    #[test]
    fn middle_outside_skips_bad_play_with_full_deck() {
        let mut s = MiddleOutside::new();
        assert_eq!(test_common::bad_opportunity(&mut s), Response::Pass);
    }

    #[test]
    fn middle_outside_plays_best_case_scenario() {
        let mut s = MiddleOutside::new();

        let response = test_common::looks_bad_is_good(&mut s, true);
        assert_eq!(response, Response::Play(BET_SIZE));
    }

    #[test]
    fn middle_outside_does_not_play_bad_opportunity_on_full_deck() {
        let mut s = MiddleOutside::new();

        let response = test_common::looks_bad_is_good(&mut s, false);
        assert_eq!(response, Response::Pass);
    }

    fn play_for_good_looking_opp(
        strategy: &mut impl Strategy,
        flip_inside_cards: bool,
    ) -> Response {
        strategy.witness(PlayEvent::Shuffle(BASE_DECK.clone()));
        if flip_inside_cards {
            let cards_to_play = BASE_DECK.iter().filter(|Card(_, value)| {
                value.to_number_ace_high() > 4 && value.to_number_ace_high() < 13
            });
            cards_to_play.for_each(|c| strategy.witness(PlayEvent::Flip(*c)));
        }
        let good_looking_opportunity = Opportunity(
            TableCard(Suit::Hearts, TableValue::Number(2)),
            TableCard(Suit::Hearts, TableValue::King),
        );
        strategy.play(&good_looking_opportunity, 1000, 1000)
    }

    #[test]
    fn middle_outside_plays_good_opp() {
        let mut s = MiddleOutside::new();

        let response = play_for_good_looking_opp(&mut s, false);
        assert_eq!(response, Response::Play(BET_SIZE));
    }

    #[test]
    fn middle_outside_passes_on_actually_bad_opp() {
        let mut s = MiddleOutside::with_values(Value::Number(3), Value::King);

        let response = play_for_good_looking_opp(&mut s, true);
        assert_eq!(response, Response::Pass);
    }
}

pub struct TwoAceStrategy<BSP: BetSizePolicy = ConstantBet> {
    bet_size_policy: BSP,
    n_twos: i32,
    n_aces: i32,
}

impl TwoAceStrategy {
    const BET_SIZE: i32 = 200;
    pub fn new() -> Self {
        Self {
            bet_size_policy: ConstantBet::new(Self::BET_SIZE),
            n_twos: 0,
            n_aces: 0,
        }
    }
}

impl Default for TwoAceStrategy {
    fn default() -> Self {
        Self::new()
    }
}

impl Strategy for TwoAceStrategy {
    fn witness(&mut self, event: PlayEvent) {
        match event {
            PlayEvent::Shuffle(_) => {
                self.n_aces = 0;
                self.n_twos = 0;
            }
            PlayEvent::Flip(Card(_, value)) => {
                if value == Value::Number(2) {
                    self.n_twos += 1;
                } else if value == Value::Ace {
                    self.n_aces += 1;
                }
            }
        }
    }
    fn get_name(&self) -> String {
        String::from("TwoAceStrategy")
    }
    fn call_ace(&self) -> AceChoice {
        AceChoice::Low
    }
    fn play(&self, opp: &Opportunity, pot_amount: PotAmount, bankroll: PotAmount) -> Response {
        let Opportunity(left, right) = opp;
        let remaining_aces = {
            let n = (N_SUITS
                - self.n_aces
                - opp.count_value(&TableValue::HiAce)
                - opp.count_value(&TableValue::LowAce)) as usize;
            BASE_DECK
                .iter()
                .filter(|Card(_, value)| *value == Value::Ace)
                .take(n)
        };
        let remaining_twos = {
            let n = (N_SUITS - self.n_twos - opp.count_value(&TableValue::Number(2))) as usize;
            BASE_DECK
                .iter()
                .filter(|Card(_, value)| *value == Value::Number(2))
                .take(n)
        };
        let representative_remaining_deck: Vec<Card> = BASE_DECK
            .iter()
            .filter(|c| {
                let Card(_, value) = c;
                **c != left.to_card()
                    && **c != right.to_card()
                    && *value != Value::Ace
                    && *value != Value::Number(2)
            })
            .chain(remaining_twos)
            .chain(remaining_aces)
            .cloned()
            .collect();
        let ev =
            Some(calculate_ev(left, right, &representative_remaining_deck)).filter(|ev| *ev > 0.0);
        if let Some(bet) =
            ev.and_then(|ev| self.bet_size_policy.get_bet_size(pot_amount, bankroll, ev))
        {
            Response::Play(bet)
        } else {
            Response::Pass
        }
    }
}

#[cfg(test)]
mod two_ace_test {
    use crate::models::*;
    use crate::strategies::test_common;
    use crate::strategies::*;

    const BET_SIZE: i32 = 200;

    #[test]
    fn two_ace_calls_good_play_with_full_deck() {
        let mut s = TwoAceStrategy::new();
        assert_eq!(
            test_common::good_opportunity(&mut s),
            Response::Play(BET_SIZE)
        );
    }

    #[test]
    fn two_ace_passes_bad_opportunity() {
        let mut s = TwoAceStrategy::new();
        assert_eq!(test_common::bad_opportunity(&mut s), Response::Pass);
    }

    #[test]
    fn two_ace_knows_secret_good_opp() {
        let mut s = TwoAceStrategy::new();
        assert_eq!(
            test_common::looks_bad_is_good(&mut s, true),
            Response::Play(BET_SIZE)
        );
    }
}

#[cfg(test)]
pub mod test_common {
    use crate::models::*;
    use crate::strategies::*;
    pub fn good_opportunity(s: &mut impl Strategy) -> Response {
        s.witness(PlayEvent::Shuffle(BASE_DECK.clone()));
        let good_opportunity = Opportunity(
            TableCard(Suit::Hearts, TableValue::Number(3)),
            TableCard(Suit::Hearts, TableValue::HiAce),
        );
        let pot_amount = 500;
        let bankroll = 11000;
        s.play(&good_opportunity, pot_amount, bankroll)
    }

    pub fn bad_opportunity(s: &mut impl Strategy) -> Response {
        s.witness(PlayEvent::Shuffle(BASE_DECK.clone()));
        let bad_opportunity = Opportunity(
            TableCard(Suit::Hearts, TableValue::Number(3)),
            TableCard(Suit::Hearts, TableValue::Number(5)),
        );
        let pot_amount = 500;
        let bankroll = 11000;
        s.play(&bad_opportunity, pot_amount, bankroll)
    }

    pub fn looks_bad_is_good(strategy: &mut impl Strategy, flip_outside_cards: bool) -> Response {
        strategy.witness(PlayEvent::Shuffle(BASE_DECK.clone()));
        if flip_outside_cards {
            let cards_to_play = BASE_DECK.iter().filter(|Card(_, value)| {
                value.to_number_ace_high() <= 4 || value.to_number_ace_high() >= 10
            });
            cards_to_play.for_each(|c| strategy.witness(PlayEvent::Flip(*c)));
        }
        let actually_good_opportunity = Opportunity(
            TableCard(Suit::Hearts, TableValue::Number(4)),
            TableCard(Suit::Hearts, TableValue::Number(10)),
        );
        strategy.play(&actually_good_opportunity, 1000, 1000)
    }
}
