mod fast_input;
use fast_input::FastInput;
use std::collections::{HashSet};

mod union_find;
use union_find::UnionFind;

use std::io::{prelude::*, stdout, BufWriter};

struct Canvas {
    map: Vec<u16>,
    length: usize,
    height: usize,
    parts: UnionFind,
    roots: HashSet<usize>,
    moves: Vec<Move>,
}

const EMPTY: u16 = 0;

impl Canvas {
    fn new(length: usize, height: usize) -> Self {
        Canvas {
            map: vec![EMPTY; length * height],
            length,
            height,
            parts: UnionFind::new(length * height),
            roots: HashSet::new(),
            moves: Vec::new(),
        }
    }

    #[inline]
    fn index(&mut self, x: usize, y: usize) -> &mut u16 {
        let index = self.gindex(x, y);
        &mut self.map[index]
    }

    #[inline]
    fn index_n(&self, x: usize, y: usize) -> u16 {
        self.map[self.gindex(x, y)]
    }

    #[inline]
    fn paint(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        self.moves.push((x1, y1, x2, y2));
        let move_index = self.moves.len() as u16;
        for x in x1..=x2 {
            for y in y1..=y2 {
                if *self.index(x, y) == EMPTY {
                    *self.index(x, y) = move_index;
                }
            }
        }
    }

    #[inline]
    fn erase_last(&mut self) {
        let move_index = self.moves.len() as u16;
        let (x1, y1, x2, y2) = self.moves.pop().unwrap();
        for x in x1..=x2 {
            for y in y1..=y2 {
                if self.index_n(x, y) == move_index {
                    *self.index(x, y) = EMPTY;
                }
            }
        }

        for x in x1..=x2 {
            for y in y1..=y2 {
                if self.index_n(x, y) == EMPTY {
                    self.join_around(x, y);
                }
            }
        }

    }

    fn init_parts(&mut self) {
        for x in 0..self.length {
            for y in 0..self.height {
                if self.index_n(x, y) == EMPTY {
                    self.join_around(x, y);
                }
            }
        }
    }

    #[inline]
    fn gindex(&self, x: usize, y: usize) -> usize {
        x + y * self.length
    }

    fn join_around(&mut self, x: usize, y: usize) {
        if x < self.length - 1 && self.index_n(x + 1, y) == EMPTY {
            let (old, new) = self.parts.union(self.gindex(x, y),
                                        self.gindex(x + 1, y));
            self.roots.remove(&old);
            self.roots.insert(new);
        }
        if x > 0 && self.index_n(x - 1, y) == EMPTY {
            let (old, new) = self.parts.union(self.gindex(x, y),
                                        self.gindex(x - 1, y));
            self.roots.remove(&old);
            self.roots.insert(new);
        }
        if y < self.height - 1 && self.index_n(x, y + 1) == EMPTY {
            let (old, new) = self.parts.union(self.gindex(x, y),
                                        self.gindex(x, y + 1));
            self.roots.remove(&old);
            self.roots.insert(new);
        }
        if y > 0 && self.index_n(x, y - 1) == EMPTY {
            let (old, new) = self.parts.union(self.gindex(x, y),
                                        self.gindex(x, y - 1));
            self.roots.remove(&old);
            self.roots.insert(new);
        }
        let root = self.parts.find(self.gindex(x as usize, y as usize));
        self.roots.insert(root);
    }

    fn parts(&self) -> usize {
        self.roots.len()
    }
}

type Move = (usize, usize, usize, usize);

fn main() {
    let mut inp = FastInput::new();
    let (n, m, q) = inp.next_triple::<usize, usize, usize>();

    let mut c = Canvas::new(n, m);

    for _ in 0..q {
        let (x1, y1, x2, y2): Move = inp.next_quad();
        c.paint(x1 - 1, y1 - 1, x2 - 1, y2 - 1);
    }

    c.init_parts();

    let mut answers = Vec::with_capacity(q);
    answers.push(c.parts());
    for _ in 0..q - 1 {
        c.erase_last();
        answers.push(c.parts());
    }

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for a in answers.iter().rev() {
        writeln!(writer, "{}", a).ok();
    }
}
