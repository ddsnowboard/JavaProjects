use crate::models::*;
use std::cmp::min;

pub struct BiggestBet {}

impl BetSizePolicy for BiggestBet {
    fn get_bet_size(
        &self,
        pot_size: PotAmount,
        bankroll: PotAmount,
        _ev: f64,
    ) -> Option<PotAmount> {
        if pot_size <= bankroll {
            Some(pot_size)
        } else {
            None
        }
    }
    fn get_name(&self) -> String {
        String::from("BiggestBet")
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
    fn get_bet_size(
        &self,
        pot_size: PotAmount,
        bankroll: PotAmount,
        _ev: f64,
    ) -> Option<PotAmount> {
        if self.amount <= pot_size && self.amount <= bankroll {
            Some(self.amount)
        } else {
            None
        }
    }
    fn get_name(&self) -> String {
        format!("ConstantBet({})", self.amount)
    }
}

pub struct PoorMansKelly {}

impl BetSizePolicy for PoorMansKelly {
    /// I really should derive the real Kelly fraction for this situation, but I'm going to use the
    /// one that ignores double burning. What's the worst that could happen?
    fn get_bet_size(&self, pot_size: PotAmount, bankroll: PotAmount, ev: f64) -> Option<PotAmount> {
        if bankroll < 2 && ev > 0.5 {
            return Some(1);
        }
        // Wait is this right?
        let coefficient = ev;
        Some(min(
            pot_size,
            min(bankroll, (bankroll as f64 * coefficient) as PotAmount),
        ))
    }

    fn get_name(&self) -> String {
        String::from("PoorMansKelly")
    }
}
