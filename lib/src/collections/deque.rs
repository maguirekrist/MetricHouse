

pub struct LinkedDeque<T> {
    front: Link<T>,
    back: Link<T>,
    len: uszize
}

type Link<T> = *mut Node<T>;

struct Node<T> {
    front: Link<T>,
    back: Link<T>,
    elem: T
}

