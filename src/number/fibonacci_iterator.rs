use std::convert::From;
use std::ops::Add;

pub struct Fibonacci<T> {
    curr: T,
    next: T,
}

impl<T> Fibonacci<T>
where
    T: Add<Output=T> + From<u8>,
{
    pub fn new() -> Self {
        Fibonacci { curr: T::from(0u8), next: T::from(1u8) }
    }
}

impl<T> Iterator for Fibonacci<T>
where
    T: Add<Output=T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let new_next = self.curr + self.next;

        self.curr = self.next;
        self.next = new_next;

        Some(self.curr)
    }
}
