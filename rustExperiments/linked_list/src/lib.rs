mod linked_list {
    pub struct List<T> {
        head: Box<Link<T>>
    }

    enum Link<T> {
        Empty, 
        More(Box<Node<T>>)
    }

    struct Node<T> {
        value: T, 
        next: Box<Link<T>>
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List {head: Box::new(Link::Empty)}
        }

        pub fn push_front(mut self, val: T) -> Self {
            let oldHead = self.head;
            let newNode = Node { value: val, next: oldHead };
            let newHead = Box::new(Link::More(Box::new(newNode)));
            self.head = newHead;
            self
        }
        
        pub fn pop_back(&mut self) -> Option<T> {
            match *self.head {
                Link::Empty => None,
                Link::More(ref mut node) => List::remove_back(&mut *node)
            }
        }

        fn remove_back(node: &mut Node<T>) -> Option<T> {
            None
            /*
            match *node.next {
                Empty =>
            }
            */
        }
    }

    #[test]
    fn it_works() {
        let l = List::new();
        assert!(*l.head == Link::Empty);
    }
}
