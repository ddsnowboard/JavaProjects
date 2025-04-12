use std::collections::LinkedList;
use std::iter::successors;

fn main() {
    let expected: &[&str] = &[
        "1",
        "11",
        "21",
        "1211",
        "111221",
        "312211",
        "13112221",
        "1113213211",
        "31131211131221",
        "13211311123113112211",
        "11131221133112132113212221",
        "3113112221232112111312211312113211",
        "1321132132111213122112311311222113111221131221",
        "11131221131211131231121113112221121321132132211331222113112211",
        "311311222113111231131112132112311321322112111312211312111322212311322113212221",
    ];

    // let look_and_say = successors(Some(String::from("1")), |last| Some(next_array(last)));
     let look_and_say = successors(Some(String::from("1")), |last| Some(next_ll(last)));

    look_and_say
        .zip(expected.iter())
        .for_each(|(actual, expected)| assert_eq!(actual, **expected));
}

fn next_array(i: &str) -> String {
    if i.is_empty() {
        String::new()
    } else {
        fn chomp(i: &str) -> (&str, String) {
            let mut chars = i.chars();
            let countee = chars.next().expect("You passed an empty string to chomp()");
            // There's always the first one
            let n_countees = chars.take_while(|c| *c == countee).count() + 1;
            let next_part = &i[n_countees..];
            let result = format!("{}{}", n_countees, countee);
            (next_part, result)
        }
        let (next_part, mut this_result) = chomp(i);
        this_result.push_str(&next_array(next_part));
        this_result
    }
}

fn next_ll(i: &str) -> String {
    if i.is_empty() {
        String::new()
    } else {
        fn inner_rec(i: &str) -> LinkedList<[u8; 2]> {
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
            let mut this_result = LinkedList::from([this_result]);
            if !next_part.is_empty() {
                this_result.append(&mut inner_rec(next_part));
            }
            this_result
        }
        let ll = inner_rec(i);
        let mut output: Vec<u8> = Vec::with_capacity(2 * ll.len());
        for arr in ll.into_iter() {
            output.push(arr[0]);
            output.push(arr[1]);
        }
        unsafe { String::from_utf8_unchecked(output) }
    }
}
