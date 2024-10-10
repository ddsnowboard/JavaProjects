use std::cmp::max;

fn main() {
    let v = vec![
        1, 3, 4, 5, 1, 25, 1, 2, 4512, 431, 52, 1, 241, 23, 1, 3241, 41, 25, 12, 51, 23, 213, 1,
        24, 1212, 51, 23, 12, 4, 21,
    ];
    println!("{}", max_profit(v));
}
// This is leetcode 309: Max Profit

#[derive(Debug)]
struct IncomeResult {
    have_stock: i32,
    no_stock: i32,
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut results_backwards: Vec<IncomeResult> = vec![];
    prices.iter().rev().for_each(|&todays_price| {
        let tomorrow = results_backwards.last();
        let two_days_from_now = results_backwards.iter().rev().nth(1);
        // If I have the stock, I can do nothing (get tomorrow's have_stock value) or sell (get
        // today's price and get 2 days from now's no_stock value, since I can't do anything
        // tomorrow).
        // If I don't have the stock, I can do nothing (get tomorrow's no_stock value) or buy
        // (spend today's price and get the have_stock value from tomorrow
        let (have_stock, no_stock) = match (tomorrow, two_days_from_now) {
            (Some(t), Some(f)) => (
                max(t.have_stock, todays_price + f.no_stock),
                max(t.no_stock, t.have_stock - todays_price),
            ),
            (Some(t), None) => (
                max(t.have_stock, todays_price),
                max(t.have_stock - todays_price, t.no_stock),
            ),
            (None, None) => (max(0, todays_price), max(-todays_price, 0)),
            (None, Some(_)) => panic!("This makes no sense"),
        };
        results_backwards.push(IncomeResult {
            have_stock,
            no_stock,
        });
    });
    results_backwards.last().unwrap().no_stock
}

#[test]
fn given_tests() {
    assert_eq!(max_profit(vec![1, 2, 3, 0, 2]), 3);
    assert_eq!(max_profit(vec![1]), 0);
}
