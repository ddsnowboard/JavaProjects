#[cfg(test)]
mod test {
    use super::super::linked_list;
    use super::super::linked_list::List;

    use std::fmt::Debug;

    #[test]
    fn basic() {
        let mut l = List::default();
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
        let mut l = List::default();
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
        let mut l: List<String> = List::default();
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
        let mut l: List<String> = List::default();
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
        let mut common_tail = List::default();
        for el in common_tail_vec.iter().rev() {
            common_tail.push_front(el.to_owned());
        }
        let mut head1 = List::default();
        head1.push_front("apples".to_owned());
        let mut head2 = List::default();
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
        let mut list = List::default();
        list.push_back("apples".to_owned());
        let front_ref = list.peek_front();
        assert_eq!(*front_ref.unwrap().get_ref(), "apples".to_owned());
    }

    #[test]
    fn test_peek_front_empty() {
        let list: List<String> = List::default();
        let front_ref = list.peek_front();
        assert!(front_ref.is_none());
    }

    #[test]
    fn test_peeks_stop_pop() {
        let mut list = List::default();
        list.push_back("apples".to_owned());
        let _front_ref1 = list.peek_front().unwrap();
        let _ptr1 = _front_ref1.get_ref_mut();
        assert_eq!(list.pop_front(), Err(linked_list::PopError::SplitOwnership));
    }

    #[test]
    #[should_panic]
    fn test_many_writers_panics() {
        let mut list = List::default();
        list.push_back("apples".to_owned());
        let _front_ref1 = list.peek_front().unwrap();
        let _ptr1 = _front_ref1.get_ref_mut();
        let _front_ref2 = list.peek_front().unwrap();
        let _ptr2 = _front_ref2.get_ref_mut();
    }

    #[test]
    fn peek_back() {
        let mut list = List::default();
        list.push_back("apples".to_owned());
        list.push_back("are".to_owned());
        list.push_back("fun".to_owned());
        assert_eq!(*list.peek_back().unwrap().get_ref(), "fun".to_owned());
    }

    #[test]
    fn test_both_front_peeks() {
        let mut list = List::default();
        list.push_back("apples".to_owned());
        let front_ref = list.peek_front().unwrap();
        {
            let ptr = front_ref.get_ref();
            assert_eq!(*ptr, "apples".to_owned());
        }

        let mut_ref = list.peek_front();
        *mut_ref.unwrap().get_ref_mut() = "pears".to_owned();

        let front_ref = list.peek_front();
        assert_eq!(*front_ref.unwrap().get_ref(), "pears".to_owned());
    }

    #[test]
    fn test_peek_front_mut() {
        let mut list = List::default();
        list.push_back("apples".to_owned());
        {
            let front_ref = list.peek_front();
            let ptr = front_ref.unwrap();
            assert_eq!(*ptr.get_ref_mut(), "apples".to_owned());
            *ptr.get_ref_mut() = "pears".to_owned();
        }
        let head = list.pop_front();
        match head {
            Ok(s) => assert_eq!(s, "pears".to_owned()),
            Err(e) => panic!("Got error: {:?}", e),
        }
    }

    #[test]
    fn test_from_nth_peek() {
        let list = list_from_slice_vec(vec!["apples", "are", "bad", "at", "coding"]);
        let new_list = list.from_nth(1);
        assert_eq!(*new_list.peek_front().unwrap().get_ref(), "are".to_owned());
    }

    #[test]
    fn test_from_nth_change() {
        let mut list = List::default();
        list.push_back("apples".to_owned());
        list.push_back("are".to_owned());
        list.push_back("bad".to_owned());
        list.push_back("at".to_owned());
        list.push_back("coding".to_owned());

        {
            let new_list = list.from_nth(1);
            *new_list.peek_front().unwrap().get_ref_mut() = "were".to_owned();
        }

        assert_eq!(list.pop_front().unwrap(), "apples".to_owned());
        assert_eq!(list.pop_front().unwrap(), "were".to_owned());
    }

    #[test]
    fn test_from_nth_push_front() {
        let mut list = list_from_slice_vec(vec!["apples", "are", "bad", "at", "coding"]);

        let mut new_list = list.from_nth(1);
        new_list.push_front("yams".to_owned());
        assert_eq!(*new_list.peek_front().unwrap().get_ref(), "yams".to_owned());

        assert_eq!(list.pop_front().unwrap(), "apples".to_owned());
        assert_eq!(list.pop_front(), Err(linked_list::PopError::SplitOwnership));
        assert_eq!(*list.peek_front().unwrap().get_ref(), "are".to_owned());
        std::mem::drop(new_list);
        assert_eq!(list.pop_front(), Ok("are".to_owned()));
    }

    #[test]
    fn test_from_nth_puah_back() {
        let mut list = list_from_slice_vec(vec!["apples", "are", "bad", "at", "coding"]);

        let new_list = list.from_nth(4);
        assert_eq!(
            *new_list.peek_front().unwrap().get_ref(),
            "coding".to_owned()
        );

        list.push_back("complicated".to_owned());
        assert_eq!(
            *new_list.peek_front().unwrap().get_ref(),
            "coding".to_owned()
        );
        assert_eq!(
            *new_list.peek_back().unwrap().get_ref(),
            "complicated".to_owned()
        );
        std::mem::drop(list);
        assert_eq!(
            *new_list.peek_front().unwrap().get_ref(),
            "coding".to_owned()
        );
        assert_eq!(
            *new_list.peek_back().unwrap().get_ref(),
            "complicated".to_owned()
        );
    }

    #[test]
    fn test_from_nth_singleton() {
        let mut list: List<String> = list_from_slice_vec(vec!["pears"]);
        {
            let new_list = list.from_nth(0);
            assert_eq!(
                *new_list.peek_front().unwrap().get_ref(),
                "pears".to_owned()
            );
            *new_list.peek_front().unwrap().get_ref_mut() = "were".to_owned();
        }

        assert_eq!(list.pop_front().unwrap(), "were".to_owned());
    }

    fn list_from_slice_vec(v: Vec<&str>) -> List<String> {
        list_from_vec(v.into_iter().map(|s| s.to_owned()).collect())
    }

    fn list_from_vec<T: Debug + Default>(v: Vec<T>) -> List<T> {
        let mut out = List::default();
        for val in v.into_iter() {
            out.push_back(val);
        }
        out
    }
}
