mod fast_input;

use fast_input::{FastInput, FastParse};

use std::collections::HashSet;

type Position = (usize, usize);

const ALIVE: u8 = std::u8::MAX;

struct Pumpkin {
    id: usize,
    death_day: u8,
    roots: [Position; 4],
}

struct PumpkinPatch {
    day: u8,
    size: usize,
    grid: Vec<Vec<HashSet<usize>>>,
    pumpkins: Vec<Pumpkin>,
}

impl PumpkinPatch {
    fn new(size: usize) -> Self {
        Self {
            day: 1,
            size,
            grid: vec![vec![HashSet::new(); size]; size],
            pumpkins: vec![],
        }
    }

    fn add_pumpkin(&mut self, (y, x): Position) {
        let id = self.pumpkins.len();
        self.grid[y][x].insert(id);

        self.pumpkins.push(Pumpkin {
            id,
            death_day: ALIVE,
            roots: [(y, x); 4],
        });
    }

    fn next_position((y, x): Position, (dy, dx): (i32, i32), size: usize) -> Option<Position> {
        let size = size as i32;
        let (y, x) = (y as i32 + dy, x as i32 + dx);

        if y >= 0 && y < size && x >= 0 && x < size {
            Some((y as usize, x as usize))
        } else {
            None
        }
    }

    fn next_day(&mut self) {
        let growth = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        let mut modified_cells = vec![];
        for pumpkin in self.pumpkins.iter_mut() {
            if self.day > pumpkin.death_day {
                continue;
            }

            for (pos, delta_pos) in pumpkin.roots.iter_mut().zip(growth) {
                if let Some(new_pos) = Self::next_position(*pos, delta_pos, self.size as usize) {
                    *pos = new_pos;
                    self.grid[pos.0][pos.1].insert(pumpkin.id);
                    modified_cells.push(*pos);
                } else {
                    pumpkin.death_day = self.day;
                }
            }
        }

        for (y, x) in modified_cells {
            if self.grid[y][x].len() < 2 {
                continue;
            }

            for &id in self.grid[y][x].iter() {
                let pumpkin = &mut self.pumpkins[id];
                if pumpkin.death_day == ALIVE {
                    pumpkin.death_day = self.day;
                }
            }
        }

        self.day += 1;
    }
}

fn main() {
    let inp = FastInput::new();
    let (p, d, n): (usize, u8, usize) = inp.next();
    let mut pumpkin_patch = PumpkinPatch::new(n);
    for _ in 0..p {
        pumpkin_patch.add_pumpkin(inp.next())
    }

    for _ in 0..d {
        pumpkin_patch.next_day();
    }

    for pumpkin in pumpkin_patch.pumpkins {
        if pumpkin.death_day == ALIVE {
            println!("ALIVE");
        } else {
            println!("{}", pumpkin.death_day);
        }
    }
}
