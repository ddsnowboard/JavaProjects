use crate::models::*;
use std::cmp::min;

pub struct BiggestBet {}

impl BetSizePolicy for BiggestBet {
    fn get_bet_size(&self, pot_size: PotAmount, bankroll: PotAmount, _ev: f64) -> PotAmount {
        min(pot_size, bankroll)
    }
}

pub struct ConstantBet {
    amount: PotAmount,
}

impl ConstantBet {
    pub fn new(amount: PotAmount) -> Self {
        Self { amount }
    }
}

impl BetSizePolicy for ConstantBet {
    fn get_bet_size(&self, pot_size: PotAmount, bankroll: PotAmount, _ev: f64) -> PotAmount {
        min(self.amount, min(pot_size, bankroll))
    }
}

pub struct PoorMansKelly {}

impl BetSizePolicy for PoorMansKelly {
    /// I really should derive the real Kelly fraction for this situation, but I'm going to use the
    /// one that ignores double burning. What's the worst that could happen?
    fn get_bet_size(&self, pot_size: PotAmount, bankroll: PotAmount, ev: f64) -> PotAmount {
        if bankroll < 2 && ev > 0.5 {
            return 1;
        }
        // Wait is this right?
        let coefficient = ev;
        min(
            pot_size,
            min(bankroll, (bankroll as f64 * coefficient) as PotAmount),
        )
    }
}
