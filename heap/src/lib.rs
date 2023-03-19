use std::{cmp::Reverse, collections::BinaryHeap};

pub struct MinHeap<T>
where
    T: Ord,
{
    heap: BinaryHeap<Reverse<T>>,
}

impl<T> MinHeap<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::<Reverse<T>>::new(),
        }
    }
    pub fn push(&mut self, x: T) {
        self.heap.push(Reverse(x))
    }
    pub fn pop(&mut self) -> Option<T> {
        self.heap.pop().map(|x| x.0)
    }
    pub fn peek(&self) -> Option<&T> {
        self.heap.peek().map(|x| &x.0)
    }
    pub fn len(&self) -> usize {
        self.heap.len()
    }
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}
