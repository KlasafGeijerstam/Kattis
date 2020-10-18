mod fast_input;
use fast_input::FastInput;

use std::io::Result;
fn main() -> Result<()> {
    let mut inp = FastInput::new();
    loop {
        let mut l = inp.next_as_iter::<u16>();
        if l.next().unwrap() == 0 {
            break
        }

        let cans: Vec<_> = l.collect();
        let sum: u16 = cans.iter().sum();
        let sub_set_sum: u16 = sum / 2;
        let mut memo = vec![false; sub_set_sum as usize + 1];
        memo[0] = true;
        for &num in &cans {
            for j in (num..sub_set_sum + 1).rev() {
                memo[j as usize] = memo[j as usize] || memo[j as usize - num as usize];
            }
        }
        for i in (0..sub_set_sum + 1).rev() {
            if memo[i as usize] {
                println!("{} {}", sum - i, i);
                break
            }
        }
    }
    Ok(())
}
