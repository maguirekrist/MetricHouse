use std::{cell::{Ref, RefCell}, rc::Rc};



pub struct List<T> {
    head: Link<T>,
    tail: Link<T>
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            prev: None,
            next: None
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }

    pub fn peek_front(&self) -> Option<Ref<T>>
    {
        self.head.as_ref().map(|node|
            Ref::map(node.borrow(), |node| {
                &node.elem
            })
        )
    }

    pub fn push_front(&mut self, elem: T)
    {
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head)
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T>
    {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self)
    {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn test_me()
    {
        let mut list = List::new();

        list.push_front(5);
    }
}