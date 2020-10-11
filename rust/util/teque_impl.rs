use std::collections::VecDeque;
pub struct Teque {
    left: VecDeque<u32>,
    right: VecDeque<u32>,
}

impl Teque {
    pub fn new() -> Self {
        Teque {
            left: VecDeque::with_capacity(500000),
            right: VecDeque::with_capacity(500000),
        }
    }

    pub fn push_back(&mut self, n: u32) {
        self.right.push_back(n);
        if self.right.len() > self.left.len() + 1 {
            self.left.push_back(self.right.pop_front().unwrap());
        }
    }

    pub fn push_front(&mut self, n: u32) {
        self.left.push_front(n);
        if self.left.len() > self.right.len() {
            self.right.push_front(self.left.pop_back().unwrap());
        }
    }

    pub fn push_middle(&mut self, n: u32) {
        if self.right.is_empty() {
            self.right.push_front(n);
        } else if self.right.len() == self.left.len() {
            self.right.push_front(n);
        } else {
            self.left.push_back(self.right.pop_front().unwrap());
            self.right.push_front(n);
        }
    }

    pub fn get(&self, i: usize) -> u32 {
        if i < self.left.len() {
            self.left[i]
        } else {
            self.right[i - self.left.len()]
        }
    }
}
