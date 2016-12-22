// This might mostly be for nought, but we'll see. So my current plan is to use a linked list.
// We'll make a branch for each different denomination we use, and we'll add the heads to a big
// set at the end. Then we'll count the set, and that'll be the answer.
#![allow(non_snake_case)]
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq)]
struct Link<'a, T: 'a>
{
    value: T,
    next: Option<&'a Link<'a, T>>,
}

impl<'a, T: 'a> Link<'a, T> {
    fn length(&self) -> usize {
        match self.next {
            Some(x) => 1 + x.length(),
            None => 1, 
        }
    }
}


fn main() {
    let denominations = [1, 2, 5, 10, 20, 50, 100, 200];
    const START: i32 = 200;
    println!("{}", count_ways(START, &denominations));
}



fn count_ways<'a> (amount: i32, denominations: &[i32]) -> i32 {

    let mut set : HashSet<&'a Link<i32>> = HashSet::new();
    for denomination in denominations {
        _count_ways(amount, denominations, *denomination, &mut set, None);
    }
    set.len() as i32
}

fn _count_ways<'a> (amount: i32, denominations: &[i32], addedCoin: i32, list: &mut HashSet<&'a Link<'a, i32>>, prevNode: Option<&'a Link<'a, i32>>)
{
    if amount == 0 {
        match prevNode {
            Some(x) => {list.insert(x);},
            None => {panic!("You must have called the function with 0!")},
        }
    }
    else if amount < 0 {
        return;
    }
    let newNode = Link<'a> {
        value: addedCoin,
        next: prevNode,
    };
    let newAmount = amount - addedCoin;
    for denom in denominations {
        _count_ways(newAmount, denominations, *denom, list, Some(& newNode));
    }
}


#[test]
fn testLinkedList() {
    let mut link3 = Link{value: 56, next: None};
    let mut link2 = Link{ value: 53, next: None};
    let mut link = Link {value: 55, next: None};
    assert_eq!(link.length(), 1);
    link2.next = Some(& link3);
    link.next = Some(& link2);
    assert_eq!(link.length(), 3);
}

