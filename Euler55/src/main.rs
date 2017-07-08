use std::collections::VecDeque;
extern crate num;
use num::bigint::BigUint;
use num::FromPrimitive;

fn main() {
    let amount = (1..10000).fold(0, |acc, n| if is_lychrel(n) { acc + 1 } else { acc });
    println!("Amount is {}", amount);
}

fn is_lychrel(mut n: u32) -> bool {
    const MAX_ITERATIONS: u8 = 50;
    let mut counter = MAX_ITERATIONS;
    let mut current_number = BigUint::from_u32(n).unwrap();
    while counter > 0 {
        let reverse = reverse_number(current_number);
        println!("Adding {} and {}", current_number, reverse);
        current_number = current_number + reverse;
        counter -= 1;
        if current_number == reverse_number(current_number) {
            return false;
        }
    }
    true
}

fn reverse_number(mut n: BigUint) -> BigUint {
    let mut digits = VecDeque::new();
    while n > 0 {
        digits.push_front(n % 10);
        n /= 10;
    }
    let mut out: u64 = 0;
    loop {
        let next = digits.pop_back();
        match next {
            Some(num) => { 
                let ten_x = out.checked_mul(10);
                match ten_x {
                    Some(multiplied) => { out = multiplied + num; }
                    None => { return 0; }
                }
            }
            None => { break; }
        }
    }
    out
}

#[test]
fn test_is_lychrel() {
    assert!(!is_lychrel(47));
    assert!(!is_lychrel(349));
    assert!(is_lychrel(196));
    assert!(is_lychrel(4994));
}

#[test]
fn test_reverse_number() {
    assert_eq!(reverse_number(12345), 54321);
    assert_eq!(reverse_number(555555), 555555);
    assert_eq!(reverse_number(0), 0);
    assert_eq!(reverse_number(1), 1);
}
