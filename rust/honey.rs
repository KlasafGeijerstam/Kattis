mod inp;
use inp::Inp;

use std::io::Result;
const S: usize = 30;
fn main() -> Result<()> {
    let mut inp = Inp::new();
    let t = inp.next();
    let mut tab = [[[[-1; 15]; S]; S]; S];
    tab[13][13][13][0] = 1;

    for i in 0..=14 {
        step(&mut tab, i, 13, 13, 13);
    }

    for _ in 0..t {
        let i: usize = inp.next();
        println!("{}", tab[13][13][13][i]);
    }
    Ok(())
}

fn step(tab: &mut [[[[i32; 15]; S]; S]; S], steps: u64, x: i32, y: i32, z: i32) -> i32 {
    let (xi, yi, zi) = (x as usize, y as usize, z as usize);
    if tab[xi][yi][zi][steps as usize] != -1 {
        return tab[xi][yi][zi][steps as usize];
    }
    if steps != 0 {
        let dmod = [(1, -1, 0), (-1, 1, 0), (0, 1, -1), (0, -1, 1), (1, 0, -1), (-1, 0, 1)];
        let mut s = 0;
        for (xm, ym, zm) in &dmod {
            let (nx, ny, nz) = (x + xm, y + ym, z + zm);
            if nx >= 0 && nx < 27 && ny >= 0 && ny < 27 && nz >= 0 && nz < 27 {
                s += step(tab, steps - 1, nx, ny, nz);
            }
        }
        tab[xi][yi][zi][steps as usize] = s; 
    }
    i32::max(tab[xi][yi][zi][steps as usize], 0)
}

