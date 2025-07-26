use std::{mem::MaybeUninit};

use rand::Rng;

type Link<T> = *mut Node<T>;

struct Node<T> {
    elem: MaybeUninit<T>,
    next: Link<T>,
    down: Link<T>
}

pub struct SkipList<T: Ord + Copy>
{
    layers: usize,
    len: usize,
    top: Link<T>
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>
}

impl<T: Ord + Copy> SkipList<T> {
    pub fn new() -> Self {
        let layer_count: usize = 4;

        //create sentinel nodes by layer.
        let top_node = Box::into_raw(
            Box::new(
                Node { 
                    elem: MaybeUninit::uninit(),
                    next: std::ptr::null_mut(),
                    down: std::ptr::null_mut()
                }
            )
        );

        let mut curr_top = top_node;
        
        for _ in 1..layer_count
        {
            unsafe {
                (*curr_top).down = Box::into_raw(
                    Box::new(
                        Node {
                            elem: MaybeUninit::uninit(),
                            next: std::ptr::null_mut(),
                            down: std::ptr::null_mut()
                        }
                    )
                );
                curr_top = (*curr_top).down;
            }
        }

        SkipList { layers: layer_count, len: 0, top: top_node }
    }

    fn get_layer_head(&self, i: usize) -> Link<T> {
        if i >= self.layers {
            panic!("Cannot get head for layer {}", i)
        }

        let mut top = self.top;
        let mut counter = 0;
        unsafe {
            while counter < i && !(*top).down.is_null()
            {
                top = (*top).down;
                counter += 1;
            }

            return top;
        }
    }

    pub fn add(&mut self, elem: T)
    {
        let mut rng = rand::rng();
        //so layer 0 is always the normal linked list...
        //but we work our way down when adding to get o(log n) insertions.
        let node = Box::into_raw(
            Box::new(
                Node {
                    elem: MaybeUninit::new(elem),
                    next: std::ptr::null_mut(),
                    down: std::ptr::null_mut()
                }
            )
        );

        unsafe {
            //Here is how you add in a skip list.
            //1. you do a tree/layer traversal to find the node that is at layer 0 and is smaller (in comparison / order) to the insert node (see above).
            //2. you then do a standard linked list insertion for the new node.
            //3. you do the probabilistic up-layer node climb.
            //4. done!

            //1. step 1.
            let mut prev_node = self.top; //this is always a dummy node, start from the top.
            let mut path: Vec<Link<T>> = Vec::with_capacity(self.layers);
            loop {
                let mut current_node = (*prev_node).next; // current_node is the cursor, we want to find a skip node.

                while !current_node.is_null() && (*node).elem.assume_init_ref() > (*current_node).elem.assume_init_ref() {
                    prev_node = current_node;
                    current_node = (*current_node).next;
                }

                path.push(prev_node);

                if (*prev_node).down.is_null() {
                    break;
                }

                prev_node = (*prev_node).down;
            }
            
            // we should be at the bottom layer... do step 2, insert the new node in the linked list.
            let curr_next = (*prev_node).next;
            (*node).next = curr_next;
            (*prev_node).next = node;


            let mut curr_node = node;
            //step 3, we now probabilistically bounce up.
            for (_layer, &prev_head) in path.iter().rev().skip(1).enumerate()
            {
                if !rng.random_bool(0.5) {
                    break;
                }
                let curr_next = (*prev_head).next;
                let elem_clone = (*curr_node).elem.assume_init_read();
                let node_copy = Box::into_raw(Box::new(
                    Node {
                        elem: MaybeUninit::new(elem_clone),
                        down: curr_node,
                        next: curr_next
                    }
                ));
                (*prev_head).next = node_copy;
                curr_node = node_copy;
            }

        }

        self.len += 1;
    }

    pub fn contains(&self, value: &T) -> bool {
        unsafe {
            let mut prev_node = self.top;//this is always a dummy node, start from the top.
            while !prev_node.is_null() {
                let mut current_node = (*prev_node).next; // current_node is the cursor, we want to find a skip node.

                while !current_node.is_null() && value > (*current_node).elem.assume_init_ref() {
                    prev_node = current_node;
                    current_node = (*current_node).next;
                }

                if !current_node.is_null() && (*current_node).elem.assume_init_ref() == value {
                    return true;
                }

                prev_node = (*prev_node).down;
            }

            false
        }
    }

    pub fn get(&self, _index: usize)
    {
        todo!();
    }

    pub fn iter(&self) -> Iter<'_, T> {
        unsafe {
            let layer_head = self.get_layer_head(3);
            let init_node = (*layer_head).next;
            Iter { next: init_node.as_ref() }
        }
    }
}

impl<T: Ord + Copy> Drop for SkipList<T> {
    fn drop(&mut self) {
        //We have to go layer by layer, and rebox in reverse
        unsafe {
            let mut layer = self.top;
            while !layer.is_null()
            {
                let mut node = layer;
                let next_layer = (*layer).down;
                while !node.is_null() {
                    let next_node = (*node).next;
                    drop(Box::from_raw(node));
                    node = next_node;
                }

                layer = next_layer;
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node| {
                self.next = node.next.as_ref();
                node.elem.assume_init_ref()
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use std::panic;
    use super::SkipList;

    #[test]
    fn get_heads()
    {
        let slist: SkipList<i32> = SkipList::new();

        assert_eq!(slist.get_layer_head(0).is_null(), false);
        assert_eq!(slist.get_layer_head(1).is_null(), false);
        assert_eq!(slist.get_layer_head(2).is_null(), false);
        assert_eq!(slist.get_layer_head(3).is_null(), false);

        let bad_op = panic::catch_unwind(|| {
            slist.get_layer_head(4);
        });

        assert!(bad_op.is_err());

        assert_eq!(slist.get_layer_head(0), slist.top);
        unsafe {
            assert_eq!(slist.get_layer_head(1), (*slist.top).down);
            assert_eq!(slist.get_layer_head(2), (*(*slist.top).down).down);
            assert_eq!(slist.get_layer_head(3), (*(*(*slist.top).down).down).down);
        }
    }

    #[test]
    fn basics()
    {
        let mut slist = SkipList::new();

        slist.add(5);

        assert_eq!(slist.len, 1);
        assert_eq!(slist.contains(&5), true);


        slist.add(10);
        slist.add(20);
        slist.add(50);
        slist.add(55);
        assert_eq!(slist.contains(&5), true);
        assert_eq!(slist.contains(&10), true);
        assert_eq!(slist.contains(&20), true);
        assert_eq!(slist.contains(&50), true);
        assert_eq!(slist.contains(&55), true);
        //assert_eq!(slist.contains(&7), false);
        assert_eq!(slist.len, 5);

        let mut iter = slist.iter();
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&50));
        assert_eq!(iter.next(), Some(&55));
        assert_eq!(iter.next(), None);

    }
}