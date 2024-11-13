use std::cmp::Eq;

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
            if data[index] >= data[parent] {
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

            if data[left] < data[smallest] {
                smallest = left;
            }
            if right < length && data[right] < data[smallest] {
                smallest = right;
            }
            if smallest == index {
                break;
            }
            data.swap(index, smallest);
            index = smallest;
        }
    }
}
