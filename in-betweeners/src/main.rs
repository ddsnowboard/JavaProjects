use rayon::prelude::*;
use std::collections::HashMap;

mod bet_size_policies;
mod game;
mod models;
mod strategies;
mod utils;

use crate::bet_size_policies::*;
use crate::game::*;
use crate::models::*;
use crate::strategies::*;

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
    simulate();
}

fn simulate() {
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
        /*
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
        */
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
                amounts_by_player.entry(idx).or_default().push(*amount);
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
