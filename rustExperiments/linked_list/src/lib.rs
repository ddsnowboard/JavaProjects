mod linked_list {
    pub struct List<T> {
        head: Link<T>
    }

    struct Node<T> {
        value: T, 
        next: Link<T>
    }

    type Link<T> = Option<Box<Node<T>>>;

    impl<T> List<T> {
        pub fn new() -> Self {
            List {head: None}
        }

        pub fn push_front(&mut self, val: T) {
            let new_node = Box::new(Node {
                value: val,
                next: self.head.take()
            });
            self.head = Some(new_node);
        }

        pub fn pop_back(&mut self) -> Option<T> {
            self.head.take().map(|node| {
                let node = *node;
                self.head = node.next;
                node.value
            })
        }

        pub fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| {
                &node.value
            })
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            self.head.as_mut().map(|node| {
                &mut node.value
            })
        }
    }

}

#[cfg(test)]
mod test {
    use super::linked_list::List;
    #[test]
    fn basic() {
        let mut l = List::new();
        l.push_front("apples");
        l.push_front("are");
        l.push_front("fun");
        l.push_front("To");
        l.push_front("Eat");

        assert_eq!(l.pop_back(), Some("Eat"));
        assert_eq!(l.peek(), Some(&"To"));
        assert_eq!(l.pop_back(), Some("To"));
        assert_eq!(l.pop_back(), Some("fun"));
        assert_eq!(l.pop_back(), Some("are"));

        l.push_front("rocks");
        assert_eq!(l.peek(), Some(&"rocks"));
        assert_eq!(l.pop_back(), Some("rocks"));

        assert_eq!(l.pop_back(), Some("apples"));
    }

    #[test]
    fn mutable_peek() {
        let mut l = List::new();
        l.push_front("apples".to_owned());
        {
            let apples: &mut String = l.peek_mut().unwrap();
            apples.push_str(" are fun");
        }
        assert_eq!(l.pop_back(), Some("apples are fun".to_owned()));
    }
}
