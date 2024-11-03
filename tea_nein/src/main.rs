use tea_nein::*;

fn main() {
    let rand = RandomPredictor::new();
    let numbers = string_to_numbers("wretchedness");
    println!("{:?}", numbers);
    println!("{}", rand.predict(&numbers));
}
