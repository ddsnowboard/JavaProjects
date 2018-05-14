fn duplicate_encode(word:&str)->String {
    word.chars().map(|c| count_letters(word, c)).map(|x| if x > 1 {')'} else {'('}).collect()
}

fn count_letters(word:&str, letter:char) -> u32 {
    // to_lowercase() returns an iterator because of Unicode or some shenanigans
    word.chars()
        .fold(0, |sum, c| if c.to_lowercase().next().unwrap() == letter.to_lowercase().next().unwrap() { sum + 1 } else { sum })
}

#[test]
fn run_tests() {
    assert_eq!(duplicate_encode("din"),"(((");
    assert_eq!(duplicate_encode("recede"),"()()()");
    assert_eq!(duplicate_encode("Success"),")())())","should ignore case");
    assert_eq!(duplicate_encode("(( @"),"))((");
}

#[test]
fn test_count_letters() {
    assert_eq!(5, count_letters("aaaaa", 'a'));
    assert_eq!(0, count_letters("aaaaa", 'b'));
    assert_eq!(2, count_letters("appledeapp", 'a'));
    assert_eq!(0, count_letters("", 'b'));
}
