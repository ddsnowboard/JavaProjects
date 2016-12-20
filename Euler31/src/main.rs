// This might mostly be for nought, but we'll see. So my current plan is to use a linked list.
// We'll make a branch for each different denomination we use, and we'll add the heads to a big
// set at the end. Then we'll count the set, and that'll be the answer.
#![allow(non_snake_case)]
use std::collections::HashMap;
use std::hash::Hash;
#[derive(PartialEq, Eq)]
struct Counter<T: Hash + Eq> {
    data: HashMap<T, u32>
}

impl<T: Hash + Eq> Counter<T> {
    fn put(&mut self, thing: T) -> bool {
        // Returns true if the thing was already in the Counter

        // Set up a hashset, then we can use it to know if there is a repeated sequence
        // Or I can use a linked list, but it goes backwards. That way we can insert new heads
        // for each different way we can go. Hmmmm...

        if self.contains_key(&thing)
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
    fn contains_key(&self, thing: &T) -> bool {
        self.data.contains_key(thing)
    }
    fn new() -> Counter<T> {
        Counter{data: HashMap::new()}
    }
}


fn main() {
    let denominations = [1, 2, 5, 10, 20, 50, 100, 200];
    println!("Hello, world!");
}




#[test]
fn test_Counter() {
    let mut c : Counter<&str> = Counter::new();
    assert_eq!(None, c.get(&"Apples"));
    c.put("Apples");
    assert_eq!(Some(&(1 as u32)), c.get(&"Apples"));
    for _ in 0..30 {
        c.put("Apples");
    }
    assert_eq!(Some(&(31 as u32)), c.get(&"Apples"));
    {
        let out = c.get_mut(&"Apples");
        assert_eq!(Some(&mut (31 as u32)), out);
        let mut out = out.unwrap();
        assert_eq!(&(31 as u32), out);
        *out = 100;
        assert_eq!(100, *out);
    }
    assert_eq!(100, *(c.get(&"Apples").unwrap()));
}
