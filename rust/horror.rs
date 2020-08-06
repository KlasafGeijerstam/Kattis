mod inp;
use inp::Inp;
use std::io::Result;

use std::collections::VecDeque;

fn main() -> Result<()> {
    let mut inp = Inp::new();
    let (n, _, l) = inp.next_triple();
    let mut movies = vec![u64::MAX; n];
    let horror_list: Vec<usize> = inp.next_as_iter().collect();

    for m in &horror_list {
        movies[*m] = 0;
    }

    let mut edges = vec![Vec::new(); n];
    for _ in 0..l {
        let (a, b) = inp.next_tuple::<usize>();
        edges[a].push(b);
        edges[b].push(a);
    }

    for &h in &horror_list {
        let mut visited = vec![false; n]; 
        //Breadth first and update 
        let mut queue = VecDeque::new();
        queue.push_back(h);
        
        while let Some(c) = queue.pop_front() {
            visited[c] = true;
            for &n in &edges[c] {
                if !visited[n] && movies[c] + 1 < movies[n] {
                    movies[n] = movies[c] + 1;
                    queue.push_back(n);
                }
            }
        }
    }

    let mut highest_value = 0;
    let mut highest_index = 0;

    for (i, &v) in movies.iter().enumerate() {
        if v == u64::MAX {
            highest_index = i;
            break;
        }
        if v > highest_value {
            highest_value = v;
            highest_index = i;
        }
    }
    println!("{}", highest_index);

    Ok(())
}
