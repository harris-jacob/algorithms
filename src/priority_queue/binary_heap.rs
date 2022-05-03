use super::traits::PriorityQueue;

pub struct MinOrderPQ<T> {
    /** binary heap stored as level ordered array */
    pq: Vec<T>,
}

impl<T: PartialOrd> MinOrderPQ<T> {
    fn less(&self, i: usize, j: usize) -> bool {
        return self.pq[i] < self.pq[j];
    }

    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.less(k / 2, k) {
            self.pq.swap(k, k / 2);
            k = k / 2;
        }
    }

    fn sink(&mut self, mut k: usize) {
        let n = self.pq.len();
        while 2 * k <= n {
            let mut j = 2 * k;
            if j < n && self.less(j, j + 1) {
                j += 1;
            }
            if !self.less(k, j) {
                break;
            };

            self.pq.swap(k, j);
            k = j;
        }
    }
}

impl<T: PartialOrd> PriorityQueue<T> for MinOrderPQ<T> {
    fn new() -> Self {
        MinOrderPQ { pq: Vec::new() }
    }

    fn empty(&self) -> bool {
        return self.pq.len() == 0;
    }

    fn insert(&mut self, val: T) {
        self.pq.push(val);
        self.swim(self.pq.len() - 1);
    }

    fn pop_max(&mut self) -> Option<T> {
        if self.empty() {
            return None;
        }

        let n = self.pq.len();
        self.pq.swap(0, n - 1);
        let max = self.pq.pop();
        self.sink(0);

        return max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_order_pq() {
        // Arrange
        let mut queue: MinOrderPQ<i32> = PriorityQueue::new();

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
}
