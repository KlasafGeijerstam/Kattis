use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }

    pub fn up(&self) -> Self {
        Pos::new(self.x, self.y + 1)
    }

    pub fn right(&self) -> Self {
        Pos::new(self.x + 1, self.y)
    }

    pub fn down(&self) -> Self {
        Pos::new(self.x, self.y - 1)
    }

    pub fn left(&self) -> Self {
        Pos::new(self.x - 1, self.y)
    }
}

struct Knowledge {
    cache: [Option<Response>; 4],
}

impl Knowledge {
    fn new() -> Self {
        Knowledge { cache: [None; 4] }
    }
}

#[derive(Clone, Copy)]
enum Response {
    Ok,
    Wall,
    Solved,
    Wrong,
}

struct ResponseReader {
    buffer: String,
    stdin: std::io::Stdin,
}

impl ResponseReader {
    fn new() -> Self {
        ResponseReader {
            buffer: String::new(),
            stdin: std::io::stdin(),
        }
    }

    fn read(&mut self) -> Response {
        self.buffer.clear();
        self.stdin.read_line(&mut self.buffer).ok();
        match self.buffer.as_str() {
            "ok\n" => Ok,
            "wall\n" => Wall,
            "solved\n" => Solved,
            _ => Wrong,
        }
    }
}

use Response::*;

fn main() {
    let mut reader = ResponseReader::new();
    let mut cache = HashMap::new();
    let mut visited = HashSet::new();
    let mut position = Pos::new(0, 0);
    let moves = [Pos::up, Pos::right, Pos::down, Pos::left];
    let tmoves = ["up", "right", "down", "left"];
    loop {
        visited.insert(position);
        let mut unknown: Vec<_> = moves
            .iter()
            .map(|f| f(&position))
            .zip(&tmoves)
            .zip(0..4)
            .filter(|((_, _), i)| {
                if let None = cache.entry(position).or_insert(Knowledge::new()).cache[*i] {
                    true
                } else {
                    false
                }
            })
            .collect();

        //Prio:
        //No knowledge,
        //Visited no knowledge
        unknown.sort_by(|((a, _), _), ((b, _), _)| {
            if !visited.contains(a) && visited.contains(b) {
                Ordering::Less
            } else if visited.contains(a) && !visited.contains(b) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        });

        if unknown.is_empty() {
            break;
        }

        let mut cell_cache = cache.entry(position).or_insert(Knowledge::new());

        for ((pos, t_move), i) in unknown {
            println!("{}", t_move);
            match reader.read() {
                Ok => {
                    position = pos;
                    cell_cache.cache[i] = Some(Ok);
                    break;
                }
                Wall => {
                    cell_cache.cache[i] = Some(Wall);
                }
                _ => return,
            }
        }
    }
    println!("no way out");
    match reader.read() {
        Solved => {},
        _ => panic!{"noo!"}
    }
}
