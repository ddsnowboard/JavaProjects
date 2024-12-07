use crate::models::*;
use crate::utils::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Game {
    pub discards: Vec<Card>,
    pub remaining_deck: Vec<Card>,
    pub current_pot: PotAmount,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new(strategies: Vec<Box<dyn Strategy>>) -> Self {
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

    pub fn shuffle(&mut self) {
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

    pub fn play_once(&mut self, player_idx: usize) {
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

    pub fn play(&mut self) -> GameResult {
        let player_indices_forever = (0..self.players.len()).cycle();
        for idx in player_indices_forever {
            // If everyone is broke or the game is over
            if !self.players.iter().any(|p| p.money > 0) || self.current_pot == 0 {
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

    pub fn to_result(&self) -> GameResult {
        GameResult {
            pot: self.current_pot,
            player_amounts: self.players.iter().map(|p| p.money).collect(),
        }
    }
}
