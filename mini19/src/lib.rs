#[derive(Debug)]
pub struct BinaryHeap<T: Ord> {
    carrier: Vec<T>,
}

impl<T: Ord> BinaryHeap<T> {
    pub fn new() -> Self {
        Self { carrier: vec![] }
    }

    pub fn insert(&mut self, item: T) {
        let mut i = self.carrier.len();
        self.carrier.push(item);
        while i != 0 && self.carrier[(i - 1) / 2] > self.carrier[i] {
            self.carrier.swap(i, (i - 1) / 2);
            i -= 1;
            i /= 2;
        }
    }

    pub fn extract_min(&mut self) -> Option<T> {
        match self.carrier.len() {
            0 => return None,
            1 => return self.carrier.pop(),
            _ => (),
        }

        let n = self.carrier.len() - 1;
        self.carrier.swap(0, n);
        let min = self.carrier.pop();
        let mut i = 0;
        loop {
            let mut j = i;
            let mut item = &self.carrier[i];
            if let Some(child) = self.carrier.get(i * 2 + 1) {
                if child < item {
                    item = child;
                    j = i * 2 + 1;
                }
            }
            if let Some(child) = self.carrier.get(i * 2 + 2) {
                if child < item {
                    j = i * 2 + 2;
                }
            }
            if i == j {
                break;
            }
            self.carrier.swap(i, j);
            i = j;
        }
        min
    }

    pub fn peek_min(&self) -> Option<&T> {
        self.carrier.get(0)
    }

    pub fn size(&self) -> usize {
        self.carrier.len()
    }

    pub fn is_empty(&self) -> bool {
        self.carrier.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut heap = BinaryHeap::<i32>::new();

        heap.insert(0);
        assert_eq!(Some(0), heap.peek_min().copied());
        heap.insert(-1);
        assert_eq!(Some(-1), heap.peek_min().copied());
        heap.insert(5);
        assert_eq!(Some(-1), heap.peek_min().copied());
        println!("{heap:?}");

        assert_eq!(Some(-1), heap.peek_min().copied());
        assert_eq!(Some(-1), heap.extract_min());
        println!("{heap:?}");
        assert_eq!(Some(0), heap.peek_min().copied());
        assert_eq!(Some(0), heap.extract_min());
        assert_eq!(Some(5), heap.peek_min().copied());
        assert_eq!(Some(5), heap.extract_min());
        assert_eq!(None, heap.peek_min());
        assert_eq!(None, heap.extract_min());
    }
}
