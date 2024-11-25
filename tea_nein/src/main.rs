use tea_nein::*;

fn main() {
    // let rand = RandomPredictor::new();
    let rand = HMM::new();
    let numbers = string_to_numbers("wretchedness");
    println!("{:?}", numbers);
    println!("{}", rand.predict(&numbers));
}

#[cfg(test)]
mod tests {
    use tea_nein::*;
    use test_case::test_case;
    #[test_case("apple")]
    #[test_case("snap")]
    #[test_case("abdicates")]
    #[test_case("beetle")]
    #[test_case("wretchedness")]
    fn generates_word(s: &str) {
        let predictor = RandomPredictor::new();
        let numbers = string_to_numbers(&s);
        println!("{:?}", numbers);
        assert_eq!(predictor.predict(&numbers), s);
    }
}
