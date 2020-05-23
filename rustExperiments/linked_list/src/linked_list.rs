use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;

use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum PopError {
    EmptyList,
    SplitOwnership,
}

#[derive(Default)]
pub struct List<T: Debug> {
    head: Link<T>,
}

#[derive(Debug)]
pub struct Node<T: Debug> {
    value: T,
    next: Link<T>,
}

pub struct MutableValueHandle<'a, T: Debug> {
    nr: NodeRef<T>,
    rm: Option<MutableValueRef<'a, T>>,
}

impl<'a, T: Debug> Deref for MutableValueHandle<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.rm.get_or_insert_with(|| {
            MutableValueRef::<'a, T>(RefMut::map(self.nr.borrow_mut(), |node| &mut node.value))
        })
    }
}

impl<'a, T: Debug> DerefMut for MutableValueHandle<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let out: &mut Self::Target =
            &mut MutableValueRef(RefMut::map(self.nr.borrow_mut(), |node| &mut node.value));
        out
    }
}

pub struct MutableValueRef<'a, T: Debug>(RefMut<'a, T>);

impl<'a, T: Debug> Deref for MutableValueRef<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'a, T: Debug> DerefMut for MutableValueRef<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

type Link<T> = Option<NodeRef<T>>;
#[derive(Debug)]
pub struct NodeRef<T: Debug>(Rc<RefCell<Node<T>>>);

impl<T: Debug> NodeRef<T> {
    pub fn get_ref(&self) -> Ref<T> {
        Ref::map(self.borrow(), |node| &node.value)
    }

    fn get_node(&self) -> Ref<Node<T>> {
        self.borrow()
    }

    pub fn get_ref_mut(&self) -> RefMut<T> {
        RefMut::map(self.borrow_mut(), |node| &mut node.value)
    }

    fn get_node_mut(&self) -> RefMut<Node<T>> {
        self.borrow_mut()
    }

    fn take_rc(self) -> Rc<RefCell<Node<T>>> {
        self.0
    }
}

impl<T: Debug> Clone for NodeRef<T> {
    fn clone(&self) -> Self {
        NodeRef(Rc::clone(&self.0))
    }
}

impl<T: Debug> Deref for NodeRef<T> {
    type Target = Rc<RefCell<Node<T>>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

enum LinkReference<'a, T: Debug> {
    First(&'a List<T>),
    Other(NodeRef<T>),
}

impl<T: Debug> List<T> {
    fn get_last_node(&self) -> Option<NodeRef<T>> {
        self.head.as_ref().map(|first| {
            let mut handle = first.clone();
            loop {
                let new_handle: Option<NodeRef<T>> = handle.borrow_mut().next.as_ref().cloned();
                if let Some(new) = new_handle {
                    handle = new;
                } else {
                    break;
                }
            }
            handle
        })
    }

    pub fn push_front(&mut self, val: T) {
        let new_node = Self::make_new_node(val, self.head.take());
        self.head = Some(new_node);
    }

    pub fn pop_front(&mut self) -> Result<T, PopError> {
        if let Some(old_front) = self.head.take() {
            match Rc::try_unwrap(old_front.take_rc()) {
                Ok(out) => {
                    let inner = out.into_inner();
                    self.head = inner.next;
                    Ok(inner.value)
                }
                Err(rc) => {
                    self.head = Some(NodeRef(rc));
                    Err(PopError::SplitOwnership)
                }
            }
        } else {
            Err(PopError::EmptyList)
        }
    }

    pub fn pop_back(&mut self) -> Result<T, PopError> {
        let last_link_holder: Option<LinkReference<T>> = self.get_last_full_link();
        if let Some(node_ref) = last_link_holder {
            match node_ref {
                LinkReference::First(_) => {
                    match Rc::try_unwrap(self.head.take().unwrap().take_rc()) {
                        Ok(out) => {
                            self.head = None;
                            Ok(out.into_inner().value)
                        }
                        Err(rc) => {
                            self.head = Some(NodeRef(rc));
                            Err(PopError::SplitOwnership)
                        }
                    }
                }
                LinkReference::Other(node_ref) => {
                    let mut borrow = node_ref.borrow_mut();
                    // We know we can do this because we got the last *full* link
                    let last_link = borrow.next.take().unwrap();
                    match Rc::try_unwrap(last_link.take_rc()) {
                        Ok(out) => {
                            borrow.next = None;
                            Ok(out.into_inner().value)
                        }
                        Err(rc) => {
                            borrow.next = Some(NodeRef(rc));
                            Err(PopError::SplitOwnership)
                        }
                    }
                }
            }
        } else {
            Err(PopError::EmptyList)
        }
    }

    pub fn push_back(&mut self, val: T) {
        let new_link = Some(Self::make_new_node(val, None));
        let last_node = self.get_last_node();
        if let Some(node_ref) = last_node {
            node_ref.borrow_mut().next = new_link;
        } else {
            self.head = new_link;
        }
    }

    fn make_new_node(val: T, next: Link<T>) -> NodeRef<T> {
        NodeRef(Rc::new(RefCell::new(Node { value: val, next })))
    }

    pub fn peek_front(&self) -> Option<NodeRef<T>> {
        self.head.as_ref().cloned()
    }

    pub fn peek_back(&self) -> Option<NodeRef<T>> {
        self.get_last_node()
    }

    fn get_last_full_link(&self) -> Option<LinkReference<T>> {
        let is_this_the_node_we_want = |node: &Node<T>| {
            node.next.is_some() && node.next.as_ref().unwrap().borrow().next.is_none()
        };

        self.head.as_ref().map(|first| {
            if first.borrow().next.is_none() {
                return LinkReference::First(self);
            }

            let mut handle: NodeRef<T> = first.clone();
            loop {
                let new_handle: Option<NodeRef<T>> = {
                    if !is_this_the_node_we_want(&handle.borrow()) {
                        Some(handle.borrow().next.as_ref().unwrap().clone())
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
            LinkReference::Other(handle)
        })
    }

    pub fn extend(&mut self, other: &List<T>) {
        let last = self.get_last_node();
        if let Some(ref tail) = other.head {
            match last {
                Some(last_ref) => last_ref.borrow_mut().next = Some(tail.clone()),
                None => self.head = Some(tail.clone()),
            }
        }
    }

    pub fn from_nth(&self, n: usize) -> List<T> {
        let mut curr_node: Link<T> = self.head.as_ref().cloned();
        for _ in 0..n {
            curr_node = curr_node
                .map(|node| node.borrow().next.as_ref().cloned())
                .flatten();
        }
        List { head: curr_node }
    }
}
