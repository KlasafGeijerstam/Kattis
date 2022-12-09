mod fast_input;
use fast_input::{FastInput, FastParse};
use std::iter::repeat;

use std::collections::VecDeque;

#[derive(Copy, Clone)]
enum Sector {
    Empty(bool),
    Residential,
    Mech,
}

use Sector::*;

type Position = (usize, usize);

#[inline]
fn can_be_hit(steps: impl Iterator<Item = Position>, map: &[Sector], width: usize) -> bool {
    for (x, y) in steps {
        match map[x + y * width] {
            Mech => {
                return true;
            }
            Residential => {
                break;
            }
            _ => {}
        }
    }

    false
}

#[inline]
fn is_game_over((gx, gy): Position, map: &[Sector], width: usize, height: usize) -> bool {
    can_be_hit((gx..width).zip(repeat(gy)), map, width)
        || can_be_hit((0..gx).rev().zip(repeat(gy)), map, width)
        || can_be_hit(repeat(gx).zip(gy..height), map, width)
        || can_be_hit(repeat(gx).zip((0..gy).rev()), map, width)
}

const MOVES: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[inline]
fn valid_moves(
    (x, y): Position,
    width: usize,
    height: usize,
) -> impl Iterator<Item = (usize, usize)> {
    MOVES
        .iter()
        .filter_map(move |(dx, dy)| {
            let (tx, ty) = (x as i32 + dx, y as i32 + dy);
            if tx >= 0 && tx < width as i32 && ty >= 0 && ty < height as i32 {
                Some((tx, ty))
            } else {
                None
            }
        })
        .map(|(x, y)| (x as usize, y as usize))
}

fn run_test_case(inp: &FastInput) {
    let (width, height): (usize, usize) = inp.next();
    let mut map = vec![Empty(false); width * height];

    let mut godzilla = (0, 0);
    let mut mechs = VecDeque::new();

    for y in 0..height {
        for (x, c) in inp.next_line().chars().enumerate() {
            map[x + y * width] = match c {
                'R' => Residential,
                'M' => {
                    mechs.push_back((x, y));
                    Mech
                }
                'G' => {
                    godzilla = (x, y);
                    Empty(true)
                }
                _ => Empty(false),
            };
        }
    }

    let mut destroyed_sectors = 0;
    loop {
        let mut godzilla_moved = false;
        for pos @ (tx, ty) in valid_moves(godzilla, width, height) {
            if let cell @ Residential = &mut map[tx + ty * width] {
                *cell = Empty(true);
                godzilla = pos;
                godzilla_moved = true;
                destroyed_sectors += 1;
                break;
            }
        }
        if !godzilla_moved {
            for pos @ (tx, ty) in valid_moves(godzilla, width, height) {
                if let Empty(visited) = map[tx + ty * width] {
                    if !visited {
                        map[tx + ty * width] = Empty(true);
                        godzilla = pos;
                        break;
                    }
                }
            }
        }
        let mech_move_count = mechs.len();
        for _ in 0..mech_move_count {
            let mech_pos = mechs.pop_back().unwrap();
            for new_pos @ (x, y) in valid_moves(mech_pos, width, height) {
                if let cell @ Empty(_) = &mut map[x + y * width] {
                    *cell = Mech;
                    mechs.push_front(new_pos);
                }
            }
        }
        if is_game_over(godzilla, &map, width, height) {
            break;
        }
    }

    println!("{}", destroyed_sectors);
}

fn main() {
    let inp = FastInput::new();
    let t = inp.next_parsed();

    for _ in 0..t {
        run_test_case(&inp);
    }
}
