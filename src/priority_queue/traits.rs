pub trait PriorityQueue<T: PartialOrd> {
    fn new() -> Self;
    fn empty(self) -> bool;
    fn insert(&mut self, val: T);
    fn pop_max(&mut self) -> Option<T>;
}
