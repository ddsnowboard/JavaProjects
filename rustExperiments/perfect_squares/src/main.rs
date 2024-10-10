use std::collections::HashMap;

fn main() {}

fn perfect_squares(i: i32) -> i32 {
    let mut d = HashMap::new();
    let p_squares = (1..).map(|x| x * x).take_while(|x| *x <= i);
    p_squares.for_each(|x| {
        d.insert(x, 1);
    });
    (2..=i).for_each(|n| {
        if !d.contains_key(&n) {
            let best = (1..n)
                .map(|left| {
                    let right = n - left;
                    assert!(left > 0);
                    assert!(right > 0);
                    let out = d[&left] + d[&right];
                    println!("Trying {} and {} to get {}", left, right, out);
                    out
                })
                .min()
                .unwrap();
            d.insert(n, best);
        }
    });
    println!("{:?}", d);
    d[&i]
}

#[test]
fn given_test() {
    assert_eq!(perfect_squares(12), 3);
    assert_eq!(perfect_squares(13), 2);
    assert_eq!(perfect_squares(196 * 2), 2);
}
