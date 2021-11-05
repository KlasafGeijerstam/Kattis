mod fast_input;
use std::collections::VecDeque;
use std::io::{BufWriter, stdout, Write};

use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    let (w, h): (usize, usize) = inp.next_tuple();
    let mut map: Vec<Vec<char>> = (0..h)
        .map(|_| inp.next_line().chars().collect())
        .collect();

    let mut visited: Vec<Vec<bool>> = (0..h).map(|_| vec![false; w]).collect();
    let command: Vec<char> = inp.next_line().chars().collect();
    
    let mut start = (0, 0);

    'outer: for y in 0..h {
        for x in 0..w {
            if map[y][x] == 'S' {
                start = (y, x);
                break 'outer;
            }
        }
    }

    visited[start.0][start.1] = true;

    let viable = search_fastest(&mut map, start, command.len());
    search(&mut map, start, &command, 0, &viable, &mut visited);

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    for l in &map {
        writeln!(writer, "{}", l.iter().collect::<String>()).ok();
    }
}
// N, E, S, W
const DIRECTIONS: [(char, i32, i32); 4] = [('N', -1, 0), ('E', 0, 1), ('S', 1, 0), ('W', 0, -1)];

fn search(map: &mut Vec<Vec<char>>,
          current_pos: (usize, usize),
          command: &Vec<char>,
          command_index: usize,
          viable: &Vec<Vec<Option<usize>>>,
          visited: &mut Vec<Vec<bool>>) {

    if viable[current_pos.0][current_pos.1].is_none() {
        return;
    }

    if viable[current_pos.0][current_pos.1].unwrap() != command_index {
        return;
    }

    if command_index == command.len() {
        map[current_pos.0][current_pos.1] = '!'; 
        return;
    }

    visited[current_pos.0][current_pos.1] = true;
     
    for (dir, y_mod, x_mod) in &DIRECTIONS {
        let (y, x) = (current_pos.0 as i32, current_pos.1 as i32);
        let new_pos = ((y + y_mod) as usize, (x + x_mod) as usize);

        if *dir != command[command_index] && !visited[new_pos.0][new_pos.1] {
            search(map, new_pos, command, command_index + 1, viable, visited);
        }
    }
}

fn search_fastest(map: &mut Vec<Vec<char>>, (start_y, start_x): (usize, usize), max_length: usize)
                                                                 -> Vec<Vec<Option<usize>>> {
    
    let mut queue = VecDeque::<(usize, usize, usize)>::new();
    queue.push_front((start_y, start_x, 0));

    let mut visited: Vec<_> = (0..map.len()).map(|_| vec![None; map[0].len()]).collect();

    visited[start_y][start_x] = Some(0);

    while !queue.is_empty() {
        let (current_y, current_x, current_len) = queue.pop_back().unwrap();

        if current_len == max_length {
            continue;
        }

        for (_, y_mod, x_mod) in &DIRECTIONS {
            let (y, x) = (current_y as i32, current_x as i32);
            let (new_y, new_x) = ((y + y_mod) as usize, (x + x_mod) as usize);

            if map[new_y][new_x] != '#' && visited[new_y][new_x].is_none() {
                visited[new_y][new_x] = Some(current_len + 1);
                queue.push_front((new_y, new_x, current_len + 1));
            }
        }
    }

    visited
}