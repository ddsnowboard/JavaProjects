mod linked_list {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct List<T> {
        head: Link<T>,
    }

    struct Node<T> {
        value: T,
        next: Link<T>,
    }

    type Link<T> = Option<NodeRef<T>>;
    type NodeRef<T> = Rc<RefCell<Node<T>>>;

    impl<T> List<T> {
        pub fn new() -> Self {
            List { head: None }
        }

        pub fn push_front(&mut self, val: T) {
            let mut new_node = Self::make_new_node(val, self.head.take());
            new_node.borrow_mut().next = self.head.take();
            self.head = Some(new_node);
        }

        pub fn pop_front(&mut self) -> Option<T> {
            if let Some(old_front) = self.head.take() {
                let handle = old_front.borrow_mut();
                self.head = Rc::clone(&handle.next);
                Some(handle.value)
            } else {
                None
            }
        }

        pub fn push_back(&mut self, val: T) {
            let last_link = self.last_link_mut();
            *last_link = Some(Self::make_new_node(val, None));
        }

        fn last_link_mut(&mut self) -> &mut Link<T> {
            fn f<V>(curr: &mut Link<V>) -> &mut Link<V> {
                if let Some(node) = curr {
                    f(&mut node.borrow_mut().next)
                } else {
                    curr
                }
            }
            f(&mut self.head)
        }

        fn last_link(&self) -> &Link<T> {
            fn f<V>(curr: &Link<V>) -> &Link<V> {
                if let Some(node) = curr {
                    f(&node.borrow().next)
                } else {
                    curr
                }
            }
            f(&self.head)
        }

        fn make_new_node(val: T, next: Link<T>) -> NodeRef<T> {
            Rc::new(RefCell::new(Node {
                value: val,
                next: next,
            }))
        }

        pub fn peek(&self) -> Option<&T> {
            self.head.as_ref().map(|node| &node.borrow().value)
        }

        pub fn peek_mut(&mut self) -> Option<&mut T> {
            self.head.as_mut().map(|node| &mut node.borrow_mut().value)
        }

        pub fn extend(&mut self, other: &List<T>) {}
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

        assert_eq!(l.pop_front(), Some("Eat"));
        assert_eq!(l.peek(), Some(&"To"));
        assert_eq!(l.pop_front(), Some("To"));
        assert_eq!(l.pop_front(), Some("fun"));
        assert_eq!(l.pop_front(), Some("are"));

        l.push_front("rocks");
        assert_eq!(l.peek(), Some(&"rocks"));
        assert_eq!(l.pop_front(), Some("rocks"));

        assert_eq!(l.pop_front(), Some("apples"));
    }

    #[test]
    fn mutable_peek() {
        let mut l = List::new();
        l.push_front("apples".to_owned());
        {
            let apples: &mut String = l.peek_mut().unwrap();
            apples.push_str(" are fun");
        }
        assert_eq!(l.pop_front(), Some("apples are fun".to_owned()));
    }

    #[test]
    fn test_push_back() {
        let mut l = List::new();
        l.push_back("Eat".to_owned());
        l.push_back("To".to_owned());
        l.push_back("fun".to_owned());
        l.push_back("are".to_owned());
        l.push_back("apples".to_owned());
        assert_eq!(
            vec!["Eat", "To", "fun", "are", "apples"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect::<Vec<String>>(),
            std::iter::from_fn(move || { l.pop_front() }).collect::<Vec<String>>()
        )
    }
}
