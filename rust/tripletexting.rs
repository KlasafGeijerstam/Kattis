use std::io;
fn main() -> io::Result<()> {
    let mut i = String::new();
    std::io::stdin().read_line(&mut i)?;
    let w: Vec<_> = i.trim().chars().collect();
    let chunks: Vec<_> = w.chunks((i.len() - 1) / 3).collect();
    for i in 0..chunks[0].len() {
        print!(
            "{}",
            if chunks[0][i] == chunks[1][i] && chunks[2][i] != chunks[0][i] {
                chunks[0][i]
            } else if chunks[1][i] == chunks[2][i] && chunks[0][i] != chunks[1][i] {
                chunks[1][i]
            } else {
                chunks[2][i]
            }
        );
    }

    println!();

    Ok(())
}
