mod inp;
use inp::Inp;
use std::io::Result;

fn main() -> Result<()> {
    let mut inp = Inp::new();

    const INF: i32 = std::i32::MAX;
    const NINF: i32 = std::i32::MIN;

    loop {
        let (n, m, q) = inp.next_triple();

        if n + m + q == 0 {
            break;
        }

        let mut dist = vec![[INF; 150]; 150];

        for i in 0..n {
            dist[i][i] = 0;
        }

        for _ in 0..m {
            let (u, v, w) = inp.next_triple::<i32>();
            if w < dist[u as usize][v as usize] {
                dist[u as usize][v as usize] = w;
            }
        }

        //Floyd the warshall on this beach
        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dist[i][k] != INF
                           && dist[k][j] != INF
                           && dist[i][j] > dist[i][k] + dist[k][j]
                    {
                        dist[i][j] = dist[i][k] + dist[k][j];
                    }
                }
            }
        }

        for k in 0..n {
            for i in 0..n {
                if dist[k][i] == NINF {
                    continue;
                }
                for j in 0..n {
                    if dist[k][j] != INF &&
                       dist[j][i] != INF &&
                       dist[j][j] < 0 
                    {
                        dist[k][i] = NINF; 
                    }
                }
            }
        }

        for _ in 0..q {
            let (u, v) = inp.next_tuple::<usize>();
            match dist[u][v] {
                INF => println!("Impossible"),
                NINF => println!("-Infinity"),
                _ => println!("{}", dist[u][v]),
            }
        }

        println!();
    }

    Ok(())
}
