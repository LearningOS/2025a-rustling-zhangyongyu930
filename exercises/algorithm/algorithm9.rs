/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.count += 1;
        let mut traverse = self.count;
        self.items.push(value);
        loop {
            if traverse == 1 {
                break;
            } else {
                let parent_index = self.parent_idx(traverse);
                let current: &T = self.items.get(traverse).unwrap();
                let parent: &T = self.items.get(parent_index).unwrap();
                if (self.comparator)(current, parent) {
                    self.items.swap(traverse, parent_index);
                    traverse = parent_index;
                } else {
                    break;
                }
            }
        };
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        } else {
            self.items.swap(1, self.count);
            let result = self.items.pop();
            self.count -= 1;
            if self.count == 0 {
                return result;
            }
            let mut traverse = 1;
            loop {
                if self.children_present(traverse) {
                    if self.right_child_idx(traverse) > self.count {
                        let current: & T = self.items.get(traverse).unwrap();
                        let left_child_index = self.left_child_idx(traverse);
                        let left_child: & T = self.items
                                                     .get(left_child_index).unwrap();
                        if (self.comparator)(current, left_child) {
                            break;
                        } else {
                            self.items.swap(traverse, left_child_index);
                            traverse = left_child_index;
                        }
                    } else {
                        let current: & T = self.items.get(traverse).unwrap();
                        let left_child_index = self.left_child_idx(traverse);
                        let right_child_index = self.right_child_idx(traverse);
                        let left_child: & T = self.items
                                                     .get(left_child_index).unwrap();
                        let right_child: & T = self.items
                                                     .get(right_child_index).unwrap();
                        if (self.comparator)(left_child, right_child) {
                            if (self.comparator)(current, left_child) {
                                break;
                            } else {
                                self.items.swap(traverse, left_child_index);
                                traverse = left_child_index;
                            }
                        } else {
                            if (self.comparator)(current, right_child) {
                                break;
                            } else {
                                self.items.swap(traverse, right_child_index);
                                traverse = right_child_index;
                            }
                        }
                    }
                } else {
                    break;
                }
            }
            result
        }
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}