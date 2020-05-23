use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

use std::fmt::Debug;

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
    rc: NodeRef<T>
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
        let last_link_holder: Option<LinkReference<T>> = self.get_last_full_link();
        if let Some(node_ref) = last_link_holder {
            match node_ref {
                LinkReference::First(_) => match Rc::try_unwrap(self.head.take().unwrap()) {
                    Ok(out) => {
                        self.head = None;
                        Ok(out.into_inner().value)
                    }
                    Err(rc) => {
                        self.head = Some(rc);
                        Err(PopError::SplitOwnership)
                    }
                },
                LinkReference::Other(node_ref) => {
                    let mut borrow = node_ref.rc.borrow_mut();
                    // We know we can do this because we got the last *full* link
                    let last_link = borrow.next.take().unwrap();
                    match Rc::try_unwrap(last_link) {
                        Ok(out) => {
                            let inner = out.into_inner();
                            borrow.next = None;
                            Ok(inner.value)
                        }
                        Err(rc) => {
                            borrow.next = Some(rc);
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

    fn get_last_full_link(&self) -> Option<LinkReference<T>> {
        let is_this_the_node_we_want =
            |node: &Node<T>| node.next.is_some() && node.next.as_ref().unwrap().borrow().next.is_none();

        self.head.as_ref().map(|first| {
                if first.borrow().next.is_none() {
                    return LinkReference::First(self);
                }

                let mut handle: NodeRef<T> = Rc::clone(first);
                loop {
                    let new_handle: Option<NodeRef<T>> = {
                        if !is_this_the_node_we_want(&handle.borrow()) {
                            Some(Rc::clone(handle.borrow().next.as_ref().unwrap()))
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
                LinkReference::Other(NodeReference {rc: handle})
        })
    }

    pub fn extend(&mut self, other: &List<T>) {
        let last = self.get_last_node();
        if let Some(ref tail) = other.head {
            match last {
                Some(last_ref) => last_ref.rc.borrow_mut().next = Some(Rc::clone(tail)),
                None => self.head = Some(Rc::clone(tail)),
            }
        }
    }
}
