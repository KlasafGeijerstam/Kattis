use std::io;
use std::collections::{HashSet, HashMap};

fn main() -> io::Result<()> {
    let n = read_ints()?;
    let mut nums = read_ints()?;
    match n[1] {
        1 => {
                nums.sort();
                let mut l = 0;
                let mut r = nums.len() - 1;
                while l < r {
                    if nums[l] != nums[r] && nums[l] + nums[r] == 7777 {
                        println!("Yes"); 
                        return Ok(());
                    } else if nums[l] + nums[r] < 7777 {
                        l += 1;
                    } else {
                        r -= 1;
                    }
                }
                println!("No");
        },
        2 => {
            println!("{}", if nums.len() == nums.iter().collect::<HashSet<&i32>>().len() {
                "Unique"    
            } else {
                "Contains duplicate"
            }); 
        },
        3 => {
            let mut m: HashMap<i32, i32> = HashMap::new();
            for k in nums {
                m.entry(k)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                if *m.get(&k).unwrap() > n[0] / 2 {
                    println!("{}", k); 
                    return Ok(());
                }
            }

            println!("-1"); 
        },
        4 => {
            nums.sort(); 
            if nums.len() % 2 == 0 {
                println!("{} {}", nums[nums.len() / 2 - 1], nums[nums.len() / 2]);
            } else {
                println!("{}", nums[nums.len() / 2]);
            }
        },
        5 => {
            let mut gr: Vec<&i32> = nums.iter().filter(|x| **x >= 100 && **x <= 999).collect(); 
            gr.sort();
            let mut s = String::new();
            let mut f = true;
            for n in gr {
                if f {
                    f = false;
                } else {
                    s.push(' ');
                }
                s.push_str(&n.to_string());
            }
            println!("{}", s);
        },
        _ => panic!("lol")
    }
    Ok(())
}

fn read_ints() -> Result<Vec<i32>, io::Error> {
    let mut k = String::new();
    io::stdin().read_line(&mut k)?;
    Ok(k.trim().split(" ").map(|x| x.parse().unwrap()).collect())
}
