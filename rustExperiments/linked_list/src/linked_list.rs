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

struct NodeReference<T: Debug> {
    rc: NodeRef<T>,
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

    fn get_node(&self) -> Ref<Node<T>> {
        self.rc.borrow()
    }
}

pub struct MutableValueReference<T: Debug> {
    rc: Rc<RefCell<Node<T>>>,
}

impl<T: Debug> MutableValueReference<T> {
    pub fn get_ref(&self) -> RefMut<T> {
        RefMut::map(self.rc.borrow_mut(), |node| &mut node.value)
    }

    fn get_node(&self) -> RefMut<Node<T>> {
        self.rc.borrow_mut()
    }
}
enum LinkReference<'a, T: Debug> {
    First(&'a List<T>),
    Other(NodeReference<T>),
}

impl<T: Debug> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    fn get_last_node(&self) -> Option<NodeReference<T>> {
        self.head.as_ref().map(|first| {
            let mut handle = Rc::clone(first);
            loop {
                let new_handle: Option<NodeRef<T>> = handle
                    .borrow_mut()
                    .next
                    .as_ref()
                    .map(|next_handle| Rc::clone(next_handle));
                if let Some(new) = new_handle {
                    handle = new;
                } else {
                    break;
                }
            }
            NodeReference { rc: handle }
        })
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
        let new_link = Some(Self::make_new_node(val, None));
        let last_node = self.get_last_node();
        if let Some(node_ref) = last_node {
            node_ref.rc.borrow_mut().next = new_link;
        } else {
            self.head = new_link;
        }
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
        let last = self.get_last_node();
        last.map(|last_noderef| ValueReference {
            rc: last_noderef.rc,
        })
    }

    pub fn peek_back_mut(&mut self) -> Option<MutableValueReference<T>> {
        self.head.as_ref().map(|noderef| MutableValueReference {
            rc: Rc::clone(noderef),
        })
    }

    pub fn extend(&mut self, other: &List<T>) {
        let last = self.get_last_node();
        if let Some(ref tail) = other.head {
            match last {
                Some(last_ref) => last_ref.rc.borrow_mut().next = Some(Rc::clone(tail)),
                None => self.head = Some(Rc::clone(tail))
            }
        }
    }
}
