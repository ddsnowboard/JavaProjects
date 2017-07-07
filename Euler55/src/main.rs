use std::collections::VecDeque;
use std::u64;

fn main() {
    println!("{}", is_lychrel(40));
}

fn is_lychrel(mut n: u64) -> bool {
    const MAX_ITERATIONS: u64 = 50;
    let mut counter = MAX_ITERATIONS;
    let mut current_number = n;
    while counter > 0 {
        let reverse = reverse_number(n);
        if n == reverse {
            return true;
        }
        n += reverse;
        counter -= 1;
    }
    false
}

fn reverse_number(mut n: u64) -> u64 {
    let mut digits = VecDeque::new();
    while n > 0 {
        digits.push_front(n % 10);
        n /= 10;
    }
    let mut out = 0;
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
    assert!(is_lychrel(47));
    assert!(is_lychrel(349));
    assert!(!is_lychrel(196));
}

#[test]
fn test_reverse_number() {
    assert_eq!(reverse_number(12345), 54321);
    assert_eq!(reverse_number(555555), 555555);
    assert_eq!(reverse_number(0), 0);
    assert_eq!(reverse_number(1), 1);
}
