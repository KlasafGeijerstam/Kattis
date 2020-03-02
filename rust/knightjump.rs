use std::collections::VecDeque;
use std::io;

fn main() -> io::Result<()> {
    let n: i32 = read()?[0];
    let size = n * n;
    let mut map: Vec<bool> = Vec::with_capacity(size as usize);
    let mut visited = [false; 10000];
    let mut buf = String::with_capacity(n as usize + 2);
    let mut start = (0, 0);
    let moves = [
        (2 as i32, -1 as i32),
        (2, 1),
        (-2, -1),
        (-2, 1),
        (1, -2),
        (1, 2),
        (-1, -2),
        (-1, 2),
    ];
    for r in 0..n {
        io::stdin().read_line(&mut buf)?;
        map.extend(buf.trim().chars().enumerate().map(|(i, x)| {
            if x == 'K' {
                start = (r as i32, i as i32);
            }
            x != '#'
        }));
        buf.clear();
    }

    let mut q = VecDeque::new();
    q.push_back((0, start));
    visited[(start.0 * n + start.1) as usize] = true;
    while !q.is_empty() {
        let (s, cur) = q.pop_front().unwrap();
        let (cx, cy) = cur;

        if cur == (0, 0) {
            println!("{}", s);
            return Ok(());
        }

        for (x, y) in &moves {
            let pos = (cx + x) * n + cy + y;
            if pos >= 0 && cx + x < n && cy + y < n && map[pos as usize] && !visited[pos as usize] {
                q.push_back((s + 1, (cx + x, cy + y)));
                visited[pos as usize] = true;
            }
        }
    }
    println!("-1");
    Ok(())
}

fn read<T>() -> io::Result<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim()
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect())
}
