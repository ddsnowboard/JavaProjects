use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::iter;

extern crate permutohedron;
use permutohedron::Heap;

fn main() {
    let input = get_input();
    let dict = get_wordset();
    let chars: Vec<_> = input.chars().collect();
    let combinator = Combinations::new(&chars);
    type ThingInBox = Vec<char>;
    // This is too complicated and doesn't work. I need to redo this part.
    let allPossibilities = combinator.fold(Box::from(iter::empty()) as Box<Iterator<Item = ThingInBox>>, |rest, nextCombo| {
        let mut combo = nextCombo.clone();
        let perms = Heap::new(&mut combo);
        let c: u32 = rest.chain(perms);
        Box::from(c) as Box<Iterator<Item = ThingInBox>>
    });
}

fn get_input() -> String { 
    use std::env::args;
    use std::process::exit;
    let mut arguments = args();
    if let Some(s) = arguments.nth(1) {
        s
    } else {
        println!("USAGE: {} LETTERS", arguments.nth(0).unwrap());
        exit(1);
    }
}

fn get_wordset() -> HashSet<String> {
    const WORDS_FILE: &'static str = "/usr/share/dict/usa";
    let mut reader = BufReader::new(File::open(WORDS_FILE).unwrap());
    let mut out = HashSet::new();

    for word in reader.lines() {
        if let Ok(s) = word {
            out.insert(s);
        } else if let Err(e) = word {
            println!("I don't understand the problem");
            panic!(e);
        }
    }
    out
}

struct Combinations<'a, T: 'a> {
    v: &'a [T],
    bits: Vec<bool>
}

impl<'a, T> Combinations<'a, T> {
    fn new(v: &'a [T]) -> Self {
        Combinations {
            v: v,
            bits: vec![false; v.len()]
        }
    }

}

fn increment_bits(bits: &mut [bool]) { 
    // Remember binary adders?
    let xor = |a, b| (a || b) && !(a && b);
    let mut carry = true;
    for v in bits.iter_mut().rev() {
        let (new_v, new_carry) = (xor(*v, carry), *v && carry);
        *v = new_v;
        carry = new_carry;
    }
}

impl<'a, T: 'a> Iterator for Combinations<'a, T> {
    type Item = Vec<&'a T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits.iter().all(|x| *x) {
            None
        } else {
            increment_bits(&mut self.bits);
            let out = self.bits.iter()
                .zip(self.v)
                .filter( |(&b, _)| b)
                .map(|(_, val)| val)
                .collect();
            Some(out)
        }
    }
}

#[test]
fn test_increment_bits() {
    let mut num = vec![false; 3];
    increment_bits(&mut num);
    assert_eq!(num, &[false, false, true]);

    increment_bits(&mut num);
    assert_eq!(num, &[false, true, false]);

    increment_bits(&mut num);
    assert_eq!(num, &[false, true, true]);

    increment_bits(&mut num);
    assert_eq!(num, &[true, false, false]);

    increment_bits(&mut num);
    assert_eq!(num, &[true, false, true]);

    increment_bits(&mut num);
    assert_eq!(num, &[true, true, false]);

    increment_bits(&mut num);
    assert_eq!(num, &[true, true, true]);

    increment_bits(&mut num);
    assert_eq!(num, &[false, false, false]);
}

#[test]
fn test_combinator() {
    const TESTER: &'static str = "acdfhjlnprsuv";
    let letters: Vec<_> = TESTER.chars().collect();
    let combinator = Combinations::new(&letters);
    for combo in combinator {
        assert!(combo.iter().all( |c| TESTER.contains(&c.to_string())));
        let mut pairs = TESTER.chars().zip(TESTER.chars().skip(1));
        assert!(pairs.all( |(a, b)| a < b));
    }
}
