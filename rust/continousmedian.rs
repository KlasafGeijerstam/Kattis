mod inp;
use inp::Inp;

use std::io::Result;
use std::collections::{BinaryHeap};
use std::cmp::Reverse;

struct MedianHeap {
    minh: BinaryHeap<Reverse<u64>>,
    maxh: BinaryHeap<u64>,
}

impl MedianHeap {
    pub fn new() -> Self {
        MedianHeap {
            minh: BinaryHeap::new(),
            maxh: BinaryHeap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.minh.is_empty() && self.maxh.is_empty() 
    }

    pub fn push(&mut self, i: u64) {
        if self.is_empty() {
            self.minh.push(Reverse(i));
        } else {
            if i <= self.median() {
                self.maxh.push(i);
            } else {
                self.minh.push(Reverse(i));
            }

            self.fix();
        }
    }

    fn fix(&mut self) {
        if i64::abs(self.minh.len() as i64 - self.maxh.len() as i64) > 1 {
            if self.maxh.len() > self.minh.len() {
                self.minh.push(Reverse(self.maxh.pop().unwrap()));
            } else {
                let Reverse(x) = self.minh.pop().unwrap();
                self.maxh.push(x)
            }
        }
    }

    pub fn median(&self) -> u64 {
        if self.minh.len() == self.maxh.len() {
            let Reverse(x) = self.minh.peek().unwrap();
            let y = self.maxh.peek().unwrap();
            (x + y) / 2
        } else if self.minh.len() > self.maxh.len() {
            let Reverse(x) = self.minh.peek().unwrap();
            *x
        } else {
            *self.maxh.peek().unwrap()
        }
    }
}

fn main() -> Result<()> {
    let mut inp = Inp::new(); 
    let t = inp.next();
    for _ in 0..t {
        let _n: usize = inp.next();
        let mut mheap = MedianHeap::new(); 
        let mut it = inp.next_as_iter::<u64>();
        mheap.push(it.next().unwrap());
        let mut s = mheap.median();
        for i in it {
            mheap.push(i);
            s += mheap.median();
        }

        println!("{}", s);

    }
    Ok(())
}
