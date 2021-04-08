use std::cell::RefCell;
use std::rc::Rc;

struct Node<T> {
    elem: T,
    pre: Link<T>,
    next: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            elem,
            pre: None,
            next: None,
        }))
    }
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        let mut new_node = Node::new(elem);
        match self.head.take() {
            None => {
                self.tail = Some(new_node.clone());
            },
            Some(mut old_head) => {
                old_head.borrow_mut().pre = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_head);

            }
        }
        self.head = Some(new_node);
    }
}