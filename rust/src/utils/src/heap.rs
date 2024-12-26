use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct HeapData<K: Ord, V> {
    key: K,
    value: V,
}

impl<K: Ord, V> PartialEq for HeapData<K, V> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<K: Ord, V> Eq for HeapData<K, V> {}

impl<K: Ord, V> PartialOrd for HeapData<K, V> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> Ord for HeapData<K, V> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        other.key.cmp(&self.key)
    }
}

#[derive(Default)]
pub struct MinHeap<K: Ord, V> {
    heap: BinaryHeap<HeapData<K, V>>,
}

impl<K: Ord, V> MinHeap<K, V> {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    #[inline]
    pub fn push(&mut self, key: K, value: V) {
        self.heap.push(HeapData { key, value });
    }

    #[inline]
    pub fn pop(&mut self) -> Option<(K, V)> {
        self.heap.pop().map(|w| (w.key, w.value))
    }

    #[inline]
    pub fn peek(&self) -> Option<(&K, &V)> {
        self.heap.peek().map(|w| (&w.key, &w.value))
    }
}
