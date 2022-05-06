use super::traits::PriorityQueue;

pub struct MaxOrderPQ<T> {
    /** binary heap stored as level ordered array */
    pq: Vec<Option<T>>,
    n: usize,
}

impl<T: PartialOrd> MaxOrderPQ<T> {
    fn less(&self, i: usize, j: usize) -> bool {
        return self.pq[i] < self.pq[j];
    }

    fn resize(&mut self, capacity: usize) {
        let mut temp = Vec::with_capacity(capacity);
        for i in 0..capacity {
            if i >= 1 && i <= self.n {
                temp.push(self.pq[i].take())
            } else {
                temp.push(None);
            }
        }

        self.pq = temp;
    }

    fn swim(&mut self, k: usize) {
        let mut k = k;
        while k > 1 && self.less(k / 2, k) {
            self.pq.swap(k, k / 2);
            k = k / 2;
        }
    }

    fn sink(&mut self, k: usize) {
        let mut k = k;
        while 2 * k <= self.n {
            let mut j = 2 * k;
            // find largest child
            if j < self.n && self.less(j, j + 1) {
                j += 1;
            }
            if self.less(k, j) {
                self.pq.swap(k, j);
            }

            k = j;
        }
    }
}

impl<T: PartialOrd> PriorityQueue<T> for MaxOrderPQ<T> {
    fn new() -> Self {
        let mut val = MaxOrderPQ {
            pq: Vec::new(),
            n: 0,
        };

        val.pq.push(None);

        return val;
    }

    fn empty(&self) -> bool {
        return self.pq.len() == 0;
    }

    fn insert(&mut self, val: T) {
        let len = self.pq.len();
        if self.n == len - 1 {
            self.resize(2 * len);
        }

        self.n += 1;
        self.pq[self.n] = Some(val);
        self.swim(self.n);
    }

    fn pop_max(&mut self) -> Option<T> {
        if self.empty() {
            return None;
        }

        self.pq.swap(self.n, 1);

        let max = self.pq[self.n].take();
        self.n -= 1;
        self.sink(1);

        let len = self.pq.len();
        if self.n > 0 && self.n == (len - 1) / 4 {
            self.resize(len / 1);
        }

        return max;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_order_pq() {
        // Arrange
        let mut queue: MaxOrderPQ<i32> = PriorityQueue::new();

        // Act
        queue.insert(1);
        queue.insert(5);
        queue.insert(3);
        queue.insert(6);
        queue.insert(2);
        queue.insert(8);
        println!("{:?}", queue.pq);

        // Assert
        assert_eq!(queue.pop_max().unwrap(), 8);
        assert_eq!(queue.pop_max().unwrap(), 6);
        assert_eq!(queue.pop_max().unwrap(), 5);
        assert_eq!(queue.pop_max().unwrap(), 3);
        assert_eq!(queue.pop_max().unwrap(), 2);
        assert_eq!(queue.pop_max().unwrap(), 1);
    }
}
