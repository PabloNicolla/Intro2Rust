use std::cmp::{Eq, Ord, Ordering, Reverse};

struct BinaryHeap<T: Eq + Ord> {
    data: Vec<T>,
}

impl<T: Eq + Ord> BinaryHeap<T> {
    pub fn new() -> Self {
        BinaryHeap { data: Vec::new() }
    }

    pub fn push(&mut self, item: T) {
        self.data.push(item);
        self.heapify_up(self.data.len() - 1);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let size = self.data.len();
        self.data.swap(0, size - 1);
        let root = self.data.pop().unwrap();
        self.heapify_down(0);
        Some(root)
    }

    fn heapify_up(&mut self, mut index: usize) {
        let data = &mut self.data;
        while index > 0 {
            let parent = (index - 1) / 2;
            if Self::compare(&data[index], &data[parent]) != Ordering::Less {
                break;
            }
            data.swap(index, parent);
            index = parent;
        }
    }

    fn heapify_down(&mut self, mut index: usize) {
        let data = &mut self.data;
        let length = data.len();
        while index * 2 + 1 < length {
            let mut smallest = index;
            let left = index * 2 + 1;
            let right = index * 2 + 2;

            if Self::compare(&data[left], &data[smallest]) == Ordering::Less {
                smallest = left;
            }
            if right < length && Self::compare(&data[right], &data[smallest]) == Ordering::Less {
                smallest = right;
            }
            if smallest == index {
                break;
            }
            data.swap(index, smallest);
            index = smallest;
        }
    }

    fn compare(a: &T, b: &T) -> Ordering {
        match (a, b) {
            (a, b) if a < b => Ordering::Less,
            (a, b) if a > b => Ordering::Greater,
            _ => Ordering::Equal,
        }
    }
}

fn main() {
    let mut max_heap = BinaryHeap::new();
    max_heap.push(5);
    max_heap.push(3);
    max_heap.push(8);

    let mut min_heap = BinaryHeap::new();
    min_heap.push(Reverse(5));
    min_heap.push(Reverse(3));
    min_heap.push(Reverse(8));

    println!("Max Heap:");
    while let Some(value) = max_heap.pop() {
        println!("{value}");
    }

    println!("Min Heap:");
    while let Some(Reverse(value)) = min_heap.pop() {
        println!("{value}");
    }
}
