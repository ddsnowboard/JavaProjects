#![allow(non_snake_case)]
use std::collections::HashMap;
use std::hash::Hash;
struct Counter<T: Hash + Eq> {
    data: HashMap<T, u32>
}

impl<T: Hash + Eq> Counter<T> {
    fn add(&mut self, thing: T) -> bool {
        // Returns true if the thing was already in the Counter

        // Set up a hashset, then we can use it to know if there is a repeated sequence
        // Or I can use a linked list, but it goes backwards. That way we can insert new heads
        // for each different way we can go. Hmmmm...

        if self.data.contains_key(&thing)
        {
            *(self.data.get_mut(&thing).unwrap()) += 1;
            true
        }
        else
        {
            self.data.insert(thing, 1);
            false
        }
    }
    fn get(&self, thing: &T) -> Option<&u32> {
        self.data.get(&thing)
    }
    fn get_mut(&mut self, thing: &T) -> Option<&mut u32> {
        self.data.get_mut(&thing)
    }
}


fn main() {
    println!("Hello, world!");
}




#[test]
fn test_Counter() {
    let c : Counter<&str> = Counter::new();

}
