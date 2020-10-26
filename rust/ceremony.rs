mod fast_input;
use fast_input::FastInput;

fn main()  {
    let mut inp = FastInput::new();
    inp.next_line();
    let mut buildings: Vec<u32> = inp.next_as_iter().collect();
    buildings.sort_unstable();
    let mut best = u32::min(buildings.len() as u32, *buildings.last().unwrap());
    for i in 1..buildings.len() {
        best = u32::min(best, (buildings.len() as u32 - i as u32) + buildings[i - 1]);
    }

    println!("{}", best);
}
