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
    /// This inner function assumes that `low_side` is less than `high_side`
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
        is_playable(left_card, right_card),
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
    assert!(
        !potential_cards.is_empty(),
        "There must be some potential cards"
    );
    potential_cards
        .iter()
        .map(|c| get_result(left_card, c, right_card))
        .map(|r| match r {
            PlayResult::Inside => 1,
            PlayResult::Outside => -1,
            PlayResult::Double => -2,
        })
        .sum::<i32>() as f64
        / potential_cards.len() as f64
}

#[cfg(test)]
mod tests {
    use crate::utils::*;
    use approx::*;
    #[test]
    fn get_result_works_for_backwards_pairs() {
        let left_card = TableCard(Suit::Hearts, TableValue::King);
        let right_card = TableCard(Suit::Spades, TableValue::Number(3));
        let test_all_possibilities = |l, r| {
            assert_eq!(
                get_result(l, &Card(Suit::Hearts, Value::Number(9)), r),
                PlayResult::Inside
            );
            assert_eq!(
                get_result(l, &Card(Suit::Hearts, Value::Ace), r),
                PlayResult::Outside
            );
            assert_eq!(
                get_result(l, &Card(Suit::Hearts, Value::King), r),
                PlayResult::Double
            );
        };
        test_all_possibilities(&left_card, &right_card);
        test_all_possibilities(&right_card, &left_card);
    }

    #[test]
    fn all_aces_double() {
        struct TestCase {
            left: TableValue,
            right: TableValue,
        }
        let middle = Card(Suit::Spades, Value::Ace);
        let cases = [
            TestCase {
                left: TableValue::LowAce,
                right: TableValue::Number(10),
            },
            TestCase {
                left: TableValue::Number(3),
                right: TableValue::HiAce,
            },
            TestCase {
                left: TableValue::LowAce,
                right: TableValue::HiAce,
            },
        ];
        for c in cases.into_iter() {
            let left = TableCard(Suit::Hearts, c.left);
            let right = TableCard(Suit::Spades, c.right);
            assert_eq!(get_result(&left, &middle, &right), PlayResult::Double);
        }
    }
    #[test]
    fn low_ace_high_ace_is_playable() {
        assert!(is_playable(
            &TableCard(Suit::Hearts, TableValue::HiAce),
            &TableCard(Suit::Spades, TableValue::LowAce)
        ))
    }

    #[test]
    fn high_ace_high_ace_is_not_playable() {
        assert!(!is_playable(
            &TableCard(Suit::Hearts, TableValue::HiAce),
            &TableCard(Suit::Spades, TableValue::HiAce)
        ))
    }

    #[test]
    fn guaranteed_win() {
        let left = TableCard(Suit::Clubs, TableValue::Number(3));
        let right = TableCard(Suit::Clubs, TableValue::Queen);
        let cards = (4..=10)
            .map(|n| Card(Suit::Hearts, Value::Number(n)))
            .collect::<Vec<_>>();
        assert!(
            relative_eq!(calculate_ev(&left, &right, &cards), 1.0),
            "Ev was not 1"
        );
    }

    #[test]
    fn guaranteed_loss() {
        let left = TableCard(Suit::Clubs, TableValue::Number(3));
        let right = TableCard(Suit::Clubs, TableValue::Number(9));
        let cards = [Value::Number(2), Value::Jack, Value::Queen, Value::King]
            .into_iter()
            .map(|v| Card(Suit::Hearts, v))
            .collect::<Vec<_>>();
        let ev = calculate_ev(&left, &right, &cards);
        assert!(relative_eq!(ev, -1.0), "Ev was {}, not -1", ev);
    }

    #[test]
    fn guaranteed_double_burn() {
        let left = TableCard(Suit::Clubs, TableValue::Number(3));
        let right = TableCard(Suit::Clubs, TableValue::Number(9));
        let cards = [Value::Number(3), Value::Number(9), Value::Number(9)]
            .into_iter()
            .map(|v| Card(Suit::Hearts, v))
            .collect::<Vec<_>>();
        let ev = calculate_ev(&left, &right, &cards);
        assert!(relative_eq!(ev, -2.0), "Ev was {}, not -2", ev);
    }

    #[test]
    fn even_odds() {
        let left = TableCard(Suit::Clubs, TableValue::Number(3));
        let right = TableCard(Suit::Clubs, TableValue::Number(9));
        let cards = [Value::Number(4), Value::Number(5), Value::Number(10),Value::Jack]
            .into_iter()
            .map(|v| Card(Suit::Hearts, v))
            .collect::<Vec<_>>();
        let ev = calculate_ev(&left, &right, &cards);
        assert!(relative_eq!(ev, 0.0), "Ev was {}, not 0", ev);
    }
}
