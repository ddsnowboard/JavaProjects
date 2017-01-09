#![allow(non_snake_case)]

fn main() {
    let denominations = [1, 2, 5, 10, 20, 50, 100, 200];
    const START: i32 = 200;
    println!("{}", count_ways(START, &denominations));
}



fn count_ways(amount: i32, denominations: &[i32]) -> i32 {
    let mut ways = vec![0; (amount + 1) as usize];
    ways[0] = 1;
    for &denom in denominations {
        for num in denom..amount + 1 {
            ways[num as usize] += ways[(num - denom) as usize];
        }
    }
    ways[amount as usize]
}

