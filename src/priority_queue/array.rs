use super::traits::PriorityQueue;

pub struct UnorderedArrayPQ<T> {
    pq: Vec<T>,
}

pub struct OrderedArrayPQ<T> {
    pub pq: Vec<T>,
}

impl<T: PartialOrd> UnorderedArrayPQ<T> {}

impl<T: PartialOrd> PriorityQueue<T> for UnorderedArrayPQ<T> {
    fn new() -> Self {
        UnorderedArrayPQ { pq: Vec::new() }
    }

    fn empty(&self) -> bool {
        return self.pq.len() == 0;
    }

    fn insert(&mut self, val: T) {
        self.pq.push(val);
    }

    fn pop_max(&mut self) -> Option<T> {
        let mut max = 0;
        for i in 0..self.pq.len() {
            if self.pq[i] > self.pq[max] {
                max = i;
            }
        }
        let n = self.pq.len();
        self.pq.swap(max, n - 1);

        return self.pq.pop();
    }
}

impl<T: PartialOrd> PriorityQueue<T> for OrderedArrayPQ<T> {
    fn new() -> Self {
        OrderedArrayPQ { pq: Vec::new() }
    }

    fn empty(&self) -> bool {
        return self.pq.len() == 0;
    }

    fn insert(&mut self, val: T) {
        let mut idx = 0;
        for i in 0..self.pq.len() {
            if self.pq[i] > val {
                break;
            }
            idx += 1;
        }

        self.pq.insert(idx, val);
    }

    fn pop_max(&mut self) -> Option<T> {
        return self.pq.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unordered_pq() {
        // Arrange
        let mut queue: UnorderedArrayPQ<i32> = PriorityQueue::new();

        // Act
        queue.insert(1);
        queue.insert(5);
        queue.insert(3);

        // Assert
        assert_eq!(queue.pop_max().unwrap(), 5);
        assert_eq!(queue.pop_max().unwrap(), 3);
        assert_eq!(queue.pop_max().unwrap(), 1);
        assert_eq!(queue.empty(), true);
    }

    #[test]
    fn test_ordered_pq() {
        // Arrange
        let mut queue: OrderedArrayPQ<i32> = PriorityQueue::new();

        // Act
        queue.insert(1);
        queue.insert(5);
        queue.insert(3);

        // Assert
        assert_eq!(queue.pop_max().unwrap(), 5);
        assert_eq!(queue.pop_max().unwrap(), 3);
        assert_eq!(queue.pop_max().unwrap(), 0);
        assert_eq!(queue.empty(), true);
    }
}
