mod linked_list {
    pub struct List<T> {
        head: Link<T>
    }

    enum Link<T> {
        Empty, 
        More(Box<Node<T>>)
    }

    struct Node<T> {
        value: T, 
        next: List<T>
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            List {head: Link::Empty}
        }

        pub fn push(&mut self, val: T) {

        }
        
        pub fn pop(self) -> Self {

        }
    }

    #[test]
    fn it_works() {
        let l = List::new();
        assert!(l.head == Link::Empty);
    }
}
