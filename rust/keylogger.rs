mod fast_input;
use fast_input::FastInput;
use std::collections::HashMap;

fn main() {
    let inp = FastInput::new();
    inp.next_line();

    let alph = [
        "clank", "bong", "click", "tap", "poing", "clonk", "clack", "ping", "tip", "cloing", "tic",
        "cling", "bing", "pong", "clang", "pang", "clong", "tac", "boing", "boink", "cloink",
        "rattle", "clock", "toc", "clink", "tuc",
    ];
    let mut char_map: HashMap<_, _> = (0..26u8)
        .zip(alph.iter())
        .map(|(n, &sound)| (sound, ('a' as u8 + n) as char))
        .collect();
    char_map.insert("whack", ' ');
    let ans = inp.lines().fold(
        (String::with_capacity(100000), false, false),
        |(mut acc, caps, shift), sound| match sound {
            sound if char_map.contains_key(sound) => {
                acc.push(if caps ^ shift {
                    char_map[sound].to_ascii_uppercase()
                } else {
                    char_map[sound]
                });
                (acc, caps, shift)
            }
            "bump" => (acc, !caps, shift),
            "dink" => (acc, caps, true),
            "thumb" => (acc, caps, false),
            "pop" => {
                acc.pop();
                (acc, caps, shift)
            }
            _ => panic!(),
        },
    );
    println!("{}", ans.0);
}
