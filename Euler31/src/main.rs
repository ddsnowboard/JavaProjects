#![allow(non_snake_case)]
use std::collections::HashMap;
struct Set<T: Hash> {
    HashMap<T, bool> data;
}

impl Set<T> {
    fn add(thing: T) -> bool {
        // Set up a hashset, then we can use it to know if there is a repeated sequence
        // Or I can use a linked list, but it goes backwards. That way we can insert new heads
        // for each different way we can go. Hmmmm...
    }
}


fn main() {
    println!("Hello, world!");
}
