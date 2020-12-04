mod fast_input;
use fast_input::{FastInput};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Island {
    goods: u32,
    required: u32,
    supplying: Vec<(usize, u32)>,
    visited: bool,
}

impl Island {
    fn new() -> Self {
        Island {
            goods: 0,
            required: 0,
            supplying: vec![],
            visited: false,
        }
    }
}

fn main() {
    let inp = FastInput::new();
    let n = inp.next();
    let mut islands = vec![Island::new(); n];
    for i in 0..n {
        let mut nums = inp.next_as_iter();
        islands[i].required = nums.next().unwrap();
        for _ in 0..nums.next().unwrap() {
            let s = nums.next().unwrap() - 1;
            let a = nums.next().unwrap();
            islands[i].goods += a;
            islands[s as usize].supplying.push((i, a));
        }
    }

    let mut collapsed = 1;
    let mut queue = VecDeque::new();
    islands[0].visited = true;
    queue.push_front(0);
    
    while let Some(c) = queue.pop_back() {
        for (n, a) in islands[c].supplying.clone() {
            islands[n].goods -= a;
            if islands[n].goods < islands[n].required && !islands[n].visited {
                islands[n].visited = true;
                collapsed += 1; 
                queue.push_front(n);
            }
        }
    }

    println!("{}", n - collapsed);
}
