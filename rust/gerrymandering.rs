use std::io;

fn main() -> io::Result<()> {
    let l = read_ints()?;
    let p = l[0];
    let d = l[1];
    let mut districts: Vec<(i32, i32)> = (0..d).map(|_| (0,0)).collect();
    let mut v = 0;
    for _ in 0..p {
        let k = read_ints()?;
        districts[(k[0] - 1) as usize].0 += k[1];
        districts[(k[0] - 1) as usize].1 += k[2];
    }
    let mut w1 = 0;
    let mut w2 = 0;
    for (v1, v2) in districts {
        let (w, lw1, lw2) = if v1 > v2 {
            w1 += v1 - ((v1 + v2) / 2 + 1);
            w2 += v2;
            ("A", v1 - ((v1 + v2) / 2 + 1), v2)
        } else {
            w2 += v2 - ((v1 + v2) / 2 + 1);
            w1 += v1;
            ("B", v1, v2 - ((v1 + v2) / 2 + 1))
        };
        v += v1 + v2;
        println!("{} {} {}", w, lw1, lw2);
    }

    println!("{}", i32::abs(w1 - w2) as f32 / v as f32);

    Ok(())
}

fn read_ints() -> Result<Vec<i32>, io::Error> {
    let mut k = String::new();
    io::stdin().read_line(&mut k)?;
    Ok(k.trim().split(" ").map(|x| x.parse().unwrap()).collect())
}
