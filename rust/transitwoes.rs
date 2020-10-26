mod fast_input;
use fast_input::FastInput;

fn main()  {
    let mut inp = FastInput::new();
    let (s, t, _) = inp.next_triple::<u32, u32, usize>();
    let walk: Vec<u32> = inp.next_as_iter().collect();
    let bus: Vec<u32> = inp.next_as_iter().collect();
    let interval: Vec<u32> = inp.next_as_iter().collect();
    let mut time = s + walk[0];
    for ((w, b), interval) in walk.iter().skip(1).zip(bus.iter()).zip(interval.iter()) {
        time += if time % interval == 0 { 0 } else { interval - (time % interval)};
        time += b + w;
    }
    println!("{}", if time <= t { "yes" } else { "no" });
}
