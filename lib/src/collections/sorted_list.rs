

type Link<T> = *mut Node<T>;

pub struct SortedList<T: Ord>
{
    begin: Link<T>,
    //head: Link<T>
}

struct Node<T> {
    elem: T,
    next: Link<T>
}

pub struct Iter<'a, T: Ord> {
    next: Option<&'a Node<T>>
}

impl<T: Ord> SortedList<T> {
    pub fn new() -> Self {
        SortedList { begin: std::ptr::null_mut() }
    }

    pub fn peek_begin(&self) -> Option<&T> {
        if self.begin.is_null() {
            None
        } else {
            unsafe {
                Some(&(*self.begin).elem)
            }
        }
    }

    pub fn seek_end(&self) -> Option<&T> {
        if self.begin.is_null() {
            None
        } else {
            let mut curr_node = self.begin;
            unsafe {
                while !(*curr_node).next.is_null() {
                    curr_node = (*curr_node).next;
                }
                Some(&(*curr_node).elem)
            }
        }
    }

    pub fn add(&mut self, elem: T) {
        let new_node = Box::into_raw(Box::new(Node {
            elem,
            next: std::ptr::null_mut(),
        }));

        unsafe {
            if self.begin.is_null() || (*self.begin).elem >= (*new_node).elem {
                (*new_node).next = self.begin;
                self.begin = new_node;
                return;
            }

            let mut prev = self.begin;
            let mut curr = (*prev).next;

            while !curr.is_null() && (*curr).elem < (*new_node).elem {
                prev = curr;
                curr = (*curr).next;
            }

            (*new_node).next = curr;
            (*prev).next = new_node;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe {
            if self.begin.is_null() {
                None
            } else {
                let begin = Box::from_raw(self.begin);

                self.begin = begin.next;

                Some(begin.elem)
            }
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            Iter { next: self.begin.as_ref() }
        }
    }
}

impl<T: Ord> Drop for SortedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
    }
}

impl<'a, T: Ord> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}


#[cfg(test)]
mod tests {
    use super::SortedList;

    #[test]
    fn basics() {
        let mut slist = SortedList::new();

        slist.add(5);
        assert_eq!(slist.peek_begin(), Some(&5));
        assert_eq!(slist.seek_end(), Some(&5));

        slist.add(4);
        assert_eq!(slist.peek_begin(), Some(&4));
        assert_eq!(slist.seek_end(), Some(&5));
        slist.add(6);
        assert_eq!(slist.peek_begin(), Some(&4));
        assert_eq!(slist.seek_end(), Some(&6));

        let mut iter = slist.iter();
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&6));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert_eq!(slist.pop(), Some(4));
        assert_eq!(slist.pop(), Some(5));
        assert_eq!(slist.pop(), Some(6));
        assert_eq!(slist.pop(), None);
    }
}