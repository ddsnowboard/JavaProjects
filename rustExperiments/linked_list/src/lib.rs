mod linked_list {
    use std::cell::Ref;
    use std::cell::RefCell;
    use std::cell::RefMut;
    use std::rc::Rc;

    use std::fmt::Debug;

    macro_rules! with_last_full_link {
        ($head: expr, $name: ident, $code: block) => {{
            {
                let _: &Link<T> = &$head;
            }
            match $head {
                Some(ref first) => {
                    let mut handle: NodeRef<T> = Rc::clone(first);
                    if handle.borrow().next.is_none() {
                        // If we want to take the value out of the RC, we need to drop it here
                        // otherwise it will look like many people hold the value.
                        std::mem::drop(handle);
                        #[allow(unused_mut)]
                        let mut $name: Option<&mut Link<T>> = Some(&mut $head);
                        $code
                    } else {
                        loop {
                            let new_handle: Option<NodeRef<T>> = {
                                let borrow = handle.borrow_mut();
                                let next_borrow = borrow.next.as_ref().unwrap().borrow_mut();
                                if next_borrow.next.is_some() {
                                    Some(Rc::clone(borrow.next.as_ref().unwrap()))
                                } else {
                                    None
                                }
                            };
                            if let Some(new) = new_handle {
                                handle = new;
                            } else {
                                break;
                            }
                        }
                        let mut last_handle = handle.borrow_mut();
                        #[allow(unused_mut)]
                        let mut $name: Option<&mut Link<T>> = Some(&mut last_handle.next);
                        $code
                    }
                }
                None => {
                    #[allow(unused_mut)]
                    let mut $name: Option<&mut Link<T>> = None;
                    $code
                }
            }
        }};
    }

    macro_rules! with_last_link {
        ($head: expr, $name: ident, $code: block) => {{
            with_last_full_link!($head, last_link, {
                if let Some(ref mut link) = last_link {
                    // We know this will work because we get the last *full* link or nothing
                    let node_ref = link.as_ref().unwrap();
                    let mut borrow = node_ref.borrow_mut();
                    let $name = &mut borrow.next;
                    $code
                } else {
                    let $name = &mut $head;
                    $code
                }
            })
        }};
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum PopError {
        EmptyList,
        SplitOwnership,
    }

    pub struct List<T: Debug> {
        head: Link<T>,
    }

    #[derive(Debug)]
    struct Node<T: Debug> {
        value: T,
        next: Link<T>,
    }

    type Link<T> = Option<NodeRef<T>>;
    type NodeRef<T> = Rc<RefCell<Node<T>>>;

    pub struct ValueReference<T: Debug> {
        rc: Rc<RefCell<Node<T>>>,
    }

    impl<T: Debug> ValueReference<T> {
        pub fn get_ref(&self) -> Ref<T> {
            Ref::map(self.rc.borrow(), |node| &node.value)
        }
    }

    pub struct MutableValueReference<T: Debug> {
        rc: Rc<RefCell<Node<T>>>,
    }

    impl<T: Debug> MutableValueReference<T> {
        pub fn get_ref(&self) -> RefMut<T> {
            RefMut::map(self.rc.borrow_mut(), |node| &mut node.value)
        }
    }

    impl<T: Debug> List<T> {
        pub fn new() -> Self {
            List { head: None }
        }

        pub fn push_front(&mut self, val: T) {
            let new_node = Self::make_new_node(val, self.head.take());
            self.head = Some(new_node);
        }

        pub fn pop_front(&mut self) -> Result<T, PopError> {
            if let Some(old_front) = self.head.take() {
                match Rc::try_unwrap(old_front) {
                    Ok(out) => {
                        let inner = out.into_inner();
                        self.head = inner.next;
                        Ok(inner.value)
                    }
                    Err(rc) => {
                        self.head = Some(rc);
                        Err(PopError::SplitOwnership)
                    }
                }
            } else {
                Err(PopError::EmptyList)
            }
        }

        pub fn pop_back(&mut self) -> Result<T, PopError> {
            with_last_full_link!(self.head, last_link, {
                if let Some(link) = last_link {
                    let node_ref = link.take().unwrap();
                    match Rc::try_unwrap(node_ref) {
                        Ok(out) => {
                            let inner = out.into_inner();
                            *link = None;
                            Ok(inner.value)
                        }
                        Err(rc) => {
                            *link = Some(rc);
                            Err(PopError::SplitOwnership)
                        }
                    }
                } else {
                    Err(PopError::EmptyList)
                }
            })
        }

        pub fn push_back(&mut self, val: T) {
            with_last_link!(self.head, last_link, {
                *last_link = Some(Self::make_new_node(val, None));
            });
        }

        fn make_new_node(val: T, next: Link<T>) -> NodeRef<T> {
            Rc::new(RefCell::new(Node { value: val, next }))
        }

        pub fn peek_front(&self) -> Option<ValueReference<T>> {
            self.head.as_ref().map(|noderef| ValueReference {
                rc: Rc::clone(noderef),
            })
        }

        pub fn peek_front_mut(&mut self) -> Option<MutableValueReference<T>> {
            self.head.as_ref().map(|noderef| MutableValueReference {
                rc: Rc::clone(noderef),
            })
        }

        pub fn peek_back(&self) -> Option<ValueReference<T>> {
            with_last_full_link!(self.head, last, {
                last.map(|last_link| {
                    let last_node_rc = last_link.as_ref().unwrap();
                    ValueReference {
                        rc: Rc::clone(last_node_rc),
                    }
                })
            })
        }

        pub fn peek_back_mut(&mut self) -> Option<MutableValueReference<T>> {
            self.head.as_ref().map(|noderef| MutableValueReference {
                rc: Rc::clone(noderef),
            })
        }

        pub fn extend(&mut self, other: &List<T>) {
            if let Some(ref tail) = other.head {
                with_last_link!(self.head, last, {
                    *last = Some(Rc::clone(tail));
                });
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::linked_list;
    use super::linked_list::List;
    #[test]
    fn basic() {
        let mut l = List::new();
        l.push_front("apples");
        l.push_front("are");
        l.push_front("fun");
        l.push_front("To");
        l.push_front("Eat");

        assert_eq!(l.pop_front().unwrap(), "Eat".to_owned());
        assert_eq!(l.pop_front().unwrap(), "To".to_owned());
        assert_eq!(l.pop_front().unwrap(), "fun".to_owned());
        assert_eq!(l.pop_front().unwrap(), "are".to_owned());

        l.push_front("rocks");
        assert_eq!(l.pop_front().unwrap(), "rocks".to_owned());

        assert_eq!(l.pop_front().unwrap(), "apples".to_owned());
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
            std::iter::from_fn(move || { l.pop_front().ok() }).collect::<Vec<String>>()
        )
    }

    #[test]
    fn test_pop_back() {
        let v: Vec<String> = vec!["are", "fun", "to", "eat"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let mut l: List<String> = List::new();
        for s in v.iter() {
            l.push_back(s.to_owned());
        }
        for s in v.iter().rev() {
            assert_eq!(&l.pop_back().unwrap(), s);
        }
        assert_eq!(l.pop_back(), Err(linked_list::PopError::EmptyList));
    }

    #[test]
    fn test_singleton_pop_back() {
        let v: Vec<String> = vec!["are"].into_iter().map(|s| s.to_owned()).collect();
        let mut l: List<String> = List::new();
        for s in v.iter() {
            l.push_back(s.to_owned());
        }
        for s in v.iter().rev() {
            assert_eq!(&l.pop_back().unwrap(), s);
        }
        assert_eq!(l.pop_back(), Err(linked_list::PopError::EmptyList));
    }

    #[test]
    fn test_extend() {
        let common_tail_vec: Vec<String> = vec!["are", "fun", "to", "eat"]
            .into_iter()
            .map(|s| s.to_owned())
            .collect();
        let mut common_tail = List::new();
        for el in common_tail_vec.iter().rev() {
            common_tail.push_front(el.to_owned());
        }
        let mut head1 = List::new();
        head1.push_front("apples".to_owned());
        let mut head2 = List::new();
        head2.push_front("cookies".to_owned());

        head1.extend(&common_tail);
        head2.extend(&common_tail);
        assert_eq!(common_tail.pop_back(), Ok("eat".to_owned()));
        assert_eq!(common_tail.pop_back(), Ok("to".to_owned()));
        assert_eq!(common_tail.pop_back(), Ok("fun".to_owned()));
        assert_eq!(
            common_tail.pop_back(),
            Err(linked_list::PopError::SplitOwnership)
        );
        assert_eq!(head1.pop_back(), Err(linked_list::PopError::SplitOwnership));
        assert_eq!(head2.pop_back(), Err(linked_list::PopError::SplitOwnership));
        std::mem::drop(head2);
        assert_eq!(
            common_tail.pop_back(),
            Err(linked_list::PopError::SplitOwnership)
        );
        assert_eq!(head1.pop_back(), Err(linked_list::PopError::SplitOwnership));
        std::mem::drop(common_tail);
        assert_eq!(head1.pop_back(), Ok("are".to_owned()));
    }

    #[test]
    fn test_peek_front() {
        let mut list = List::new();
        list.push_back("apples".to_owned());
        let front_ref = list.peek_front();
        assert_eq!(*front_ref.unwrap().get_ref(), "apples".to_owned());
    }

    #[test]
    fn test_peek_front_empty() {
        let list: List<String> = List::new();
        let front_ref = list.peek_front();
        assert!(front_ref.is_none());
    }

    #[test]
    fn test_peeks_stop_pop() {
        let mut list = List::new();
        list.push_back("apples".to_owned());
        let _front_ref1 = list.peek_front_mut().unwrap();
        let _ptr1 = _front_ref1.get_ref();
        assert_eq!(list.pop_front(), Err(linked_list::PopError::SplitOwnership));
    }

    #[test]
    #[should_panic]
    fn test_many_writers_panics() {
        let mut list = List::new();
        list.push_back("apples".to_owned());
        let _front_ref1 = list.peek_front_mut().unwrap();
        let _ptr1 = _front_ref1.get_ref();
        let _front_ref2 = list.peek_front_mut().unwrap();
        let _ptr2 = _front_ref2.get_ref();
    }

    #[test]
    fn peek_back() {
        let mut list = List::new();
        list.push_back("apples".to_owned());
        list.push_back("are".to_owned());
        list.push_back("fun".to_owned());
        assert_eq!(*list.peek_back().unwrap().get_ref(), "fun".to_owned());
    }

    #[test]
    fn test_both_front_peeks() {
        let mut list = List::new();
        list.push_back("apples".to_owned());
        let front_ref = list.peek_front().unwrap();
        {
            let ptr = front_ref.get_ref();
            assert_eq!(*ptr, "apples".to_owned());
        }

        let mut_ref = list.peek_front_mut();
        *mut_ref.unwrap().get_ref() = "pears".to_owned();

        let front_ref = list.peek_front();
        assert_eq!(*front_ref.unwrap().get_ref(), "pears".to_owned());
    }

    #[test]
    fn test_peek_front_mut() {
        let mut list = List::new();
        list.push_back("apples".to_owned());
        {
            let front_ref = list.peek_front_mut();
            let ptr = front_ref.unwrap();
            assert_eq!(*ptr.get_ref(), "apples".to_owned());
            *ptr.get_ref() = "pears".to_owned();
        }
        let head = list.pop_front();
        match head {
            Ok(s) => assert_eq!(s, "pears".to_owned()),
            Err(e) => panic!("Got error: {:?}", e),
        }
    }
}
