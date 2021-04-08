/*use std::mem;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push(&mut self, elem: T) {
        let node = Some(Box::new(Node {
            elem,
            next: None,
        }));
        let old_tail = mem::replace(&mut self.tail, a);
        match old_tail {
            None => {
                self.head = node;
            }
            Some(_) => {}
        }
    }
}*/