use std::collections::LinkedList;
use std::iter::successors;
fn next_array(mut i: &str) -> String {
    fn chomp(i: &str) -> (&str, String) {
        let mut chars = i.chars();
        let countee = chars.next().expect("You passed an empty string to chomp()");
        // There's always the first one
        let n_countees = chars.take_while(|c| *c == countee).count() + 1;
        let next_part = &i[n_countees..];
        let result = format!("{}{}", n_countees, countee);
        (next_part, result)
    }
    let mut result = String::new();
    while !i.is_empty() {
        let (next_string, this_result) = chomp(i);
        result.push_str(&this_result);
        i = next_string;
    }
    result
}

fn make_iterator(f: impl Fn(&str) -> String + 'static) -> impl Iterator<Item = String> {
    successors(Some(String::from("1")), move |last| Some(f(last)))
}

pub fn array_iterator() -> impl Iterator<Item = String> {
    make_iterator(next_array)
}

pub fn ll_iterator() -> impl Iterator<Item = String> {
    make_iterator(next_ll)
}

pub fn next_ll(mut i: &str) -> String {
    let mut builder = LinkedList::new();

    while !i.is_empty() {
        fn chomp(i: &str) -> (&str, [u8; 2]) {
            let mut chars = i.chars();
            let countee = chars.next().expect("You passed an empty string to chomp()");
            // There's always the first one
            let n_countees = chars.take_while(|c| *c == countee).count() + 1;
            let next_part = &i[n_countees..];
            let result = [(n_countees as u8) + b'0', countee as u8];
            (next_part, result)
        }
        let (next_part, this_result) = chomp(i);
        builder.push_back(this_result);
        i = next_part;
    }
    let mut output: Vec<u8> = Vec::with_capacity(2 * builder.len());
    for arr in builder.into_iter() {
        output.push(arr[0]);
        output.push(arr[1]);
    }
    unsafe { String::from_utf8_unchecked(output) }
}
