use crate::models::*;
use std::sync::LazyLock;

pub static BASE_DECK: LazyLock<Vec<Card>> = LazyLock::new(|| {
    let all_numbers = (2..=10).map(Value::Number);
    let all_values = all_numbers.chain(vec![Value::Ace, Value::King, Value::Queen, Value::Jack]);
    let all_suits = [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs];
    all_values
        .flat_map(|v| all_suits.iter().copied().map(move |s| Card(s, v)))
        .collect::<Vec<_>>()
});

pub fn is_playable(left_card: &TableCard, right_card: &TableCard) -> bool {
    let TableCard(_, left_value) = left_card;
    let TableCard(_, right_value) = right_card;
    left_value != right_value
}

pub fn get_result(left_card: &TableCard, middle_card: &Card, right_card: &TableCard) -> PlayResult {
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

pub fn calculate_ev(
    left_card: &TableCard,
    right_card: &TableCard,
    potential_cards: &[Card],
) -> f64 {
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
