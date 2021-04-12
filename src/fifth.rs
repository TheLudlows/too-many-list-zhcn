use std::mem;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<'a, T> {
    head: Link<T>,
    tail: Option<&'a mut Node<T>>,
}

impl<'a, T> List<'a, T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn push(& 'a mut self, elem: T) {
        let node = Some(Box::new(Node {
            elem,
            next: None,
        }));

        let new_tail = match self.tail.take() {
            None => {
                self.head = node;
                self.head.as_deref_mut()
            }
            Some(old_tail) => {
                old_tail.next = node;
                old_tail.next.as_deref_mut()
            }
        };
        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            if self.head.is_none() {
                self.tail = None;
            }
            node.elem
        })
    }
}