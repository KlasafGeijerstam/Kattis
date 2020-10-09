mod inp;
use inp::Inp;

fn main() {
    let mut inp = Inp::new();
    let l = inp.next_line();
    let res: Vec<_> = l
        .trim()
        .split_ascii_whitespace()
        .filter(|s| {
            s.as_bytes().iter().all(|&b| {
                b >= 32 && b <= 47
                    || b >= 58 && b <= 64
                    || b >= 91 && b <= 96
                    || b == 109
                    || b == 117
                    || b >= 123 && b <= 126
            })
        })
        .map(|c| {
            c.chars()
                .filter(|&k| k == 'u' || k == 'm')
                .map(|k| if k == 'u' { 1u8 } else { 0u8 })
        })
        .flatten()
        .collect();
    println!(
        "{}",
        res.chunks(7)
            .map(|chunk| {
                let mut b = 0u8;
                for (i, &k) in chunk.iter().enumerate() {
                    if k == 1 {
                        b |= 1 << 6 - i;
                    }
                }
                b as char
            })
            .collect::<String>()
    );
}

