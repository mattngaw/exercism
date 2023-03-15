use std::iter::FromIterator;

pub struct SimpleLinkedList<T: Clone + Copy> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Clone)]
struct Node<T: Clone + Copy> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone + Copy> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            len: 0usize,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some 
    // types, whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.len == 0usize
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, element: T) {
        let n = Box::new(Node {
            data: element,
            next: self.head.take(),
        });
        self.head = Some(n);
        self.len += 1;
        return
    }

    pub fn pop(&mut self) -> Option<T> {
        let result = self.head.as_ref().map(|b| {
            self.len -= 1;
            b.data
        });
        self.head = self.head.take().and_then(|b| b.next);
        result
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().and_then(|b| Some(&b.data))
    }

    #[must_use]
    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut list = Self::new();
        while let Some(b) = self.pop() {
            list.push(b);
        }
        list
    }
}

impl<T: Copy> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self { 
        let mut list = Self::new();
        for element in iter {
            list.push(element);
        }
        list
    }
}

// In general, it would be preferable to implement IntoIterator for 
// SimpleLinkedList<T> instead of implementing an explicit conversion to a 
// vector. This is because, together, FromIterator and IntoIterator enable 
// conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> 
// instead of IntoIterator is that implementing that interface is fairly 
// complicated, and demands more of the student than we expect at this point in 
// the track.

impl<T: Copy> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut vec = Vec::new();
        while let Some(x) = linked_list.pop() {
            vec.insert(0, x);
        }
        vec
    }
}
