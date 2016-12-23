#![allow(non_snake_case)]
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
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

    let mut set : HashSet<Link<i32>> = HashSet::new();
    for denomination in denominations {
        _count_ways(amount, denominations, *denomination, &mut set, None);
    }
    set.len() as i32
}

fn _count_ways<'a> (amount: i32, denominations: &[i32], addedCoin: i32, list: &mut HashSet<Link<'a, i32>>, prevNode: Option<Link<'a, i32>>)
{
    if amount == 0 {
        match prevNode {
            Some(x) => {list.insert(x.clone());},
            None => {panic!("You must have called the function with 0!")},
        }
    }
    else if amount < 0 {
        return;
    }
    let otherX = match prevNode {
            Some(x) => Some(& x.clone()),
            None => None};

    // I don't understand lifetimes well enough to make 
    // this work. It says that I can't put the next node into this newNode 
    // because the next node doesn't live long enough. I don't know how to fix it. 
    let newNode = Link {
        value: addedCoin,
        next: otherX, 
    };
    let newAmount = amount - addedCoin;
    for denom in denominations {
        _count_ways(newAmount, denominations, *denom, list, Some(newNode.clone()));
    }
}


#[test]
fn testLinkedList() {
    let mut link3 = Link {value: 56, next: None};
    let mut link2 = Link {value: 53, next: None};
    let mut link = Link {value: 55, next: None};
    assert_eq!(link.length(), 1);
    link2.next = Some(& link3);
    link.next = Some(& link2);
    assert_eq!(link.length(), 3);
}

