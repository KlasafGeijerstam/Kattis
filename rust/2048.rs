mod inp;
use inp::Inp;
use std::io::Result;

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let mut board: Vec<_> = (0..4)
        .map(|_| inp.next_as_iter().collect::<Vec<u16>>())
        .collect();
    let d = inp.next();
    match d {
        0 => move_left(&mut board),
        1 => move_up(&mut board),
        2 => move_right(&mut board),
        _ => move_down(&mut board),
    }
    
    for r in board {
        for (i, c) in r.iter().enumerate() {
            print!("{}", c);
            if i != 3 {
                print!(" ");
            }
        }
        println!();
    }
     
    Ok(())
}

fn move_left(b: &mut Vec<Vec<u16>>) {
    let mut merged = [[false; 4]; 4];
    for r in 0..4 {
        for c in 1..4 {
            if b[r][c] == 0 {
                continue
            }
            for m in (0..c).rev() {
                if merged[r][m] || b[r][m] != b[r][m + 1] && b[r][m] != 0 {
                    break
                }
                if b[r][m] == b[r][m + 1] {
                    b[r][m] *= 2;
                    merged[r][m] = true;
                    b[r][m + 1] = 0;
                    break
                } 
                b[r][m] = b[r][m + 1];
                b[r][m + 1] = 0;
            }
        }
    }
}

fn move_up(b: &mut Vec<Vec<u16>>) {
    let mut merged = [[false; 4]; 4];
    for r in 1..4 {
        for c in 0..4 {
            if b[r][c] == 0 {
                continue
            }
            for m in (0..r).rev() {
                if merged[m][c] || b[m][c] != b[m + 1][c] && b[m][c] != 0 {
                    break
                }
                if b[m][c] == b[m + 1][c] {
                    b[m][c] *= 2;
                    merged[m][c] = true;
                    b[m + 1][c] = 0;
                    break
                } 
                b[m][c] = b[m + 1][c];
                b[m + 1][c] = 0;
            }
        }
    }
}

fn move_right(b: &mut Vec<Vec<u16>>) {
    let mut merged = [[false; 4]; 4];
    for r in 0..4 {
        for c in (0..3).rev() {
            if b[r][c] == 0 {
                continue
            }
            for m in c + 1..4 {
                if merged[r][m] || b[r][m] != b[r][m - 1] && b[r][m] != 0 {
                    break
                }
                if b[r][m] == b[r][m - 1] {
                    b[r][m] *= 2;
                    merged[r][m] = true;
                    b[r][m - 1] = 0;
                    break
                }
                b[r][m] = b[r][m - 1];
                b[r][m - 1] = 0;
            }
        }
    }
}

fn move_down(b: &mut Vec<Vec<u16>>) {
    let mut merged = [[false; 4]; 4];
    for r in (0..3).rev() {
        for c in 0..4 {
            if b[r][c] == 0 {
                continue
            }
            for m in r + 1..4 {
                if merged[m][c] || b[m][c] != b[m - 1][c] && b[m][c] != 0 {
                    break
                }
                if b[m][c] == b[m - 1][c] {
                    b[m][c] *= 2;
                    merged[m][c] = true;
                    b[m - 1][c] = 0;
                    break
                }
                b[m][c] = b[m - 1][c];
                b[m - 1][c] = 0;
            }
        }
    }
}


