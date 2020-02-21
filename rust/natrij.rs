use std::io;

fn main() -> io::Result<()> {
    let cur = read_ints()?;
    let amount = read_ints()?;
    let mut s = cur[0] * 3600 + cur[1] * 60 + cur[2];
    let og = s;
    loop {
        s += 1;
        let m = (s) / 60 % 60; 
        let h = ((s / 60) / 60) % 24; 
        let cs = s % 60; 
         
        if h == amount[0] && m == amount[1] && cs == amount[2] {
            s -= og;
            let m = (s) / 60 % 60; 
            let h = (s / 60) / 60; 
            let cs = s % 60; 
            println!("{:02}:{:02}:{:02}", h, m, cs);
            return Ok(());
        }
       
    }
}

fn read_ints() -> io::Result<Vec<usize>> {
    let mut k = String::new();
    io::stdin().read_line(&mut k)?;
    Ok(k.trim().split(":").map(|x| x.parse().unwrap()).collect())
}
