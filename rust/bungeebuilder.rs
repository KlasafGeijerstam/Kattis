mod fast_input;
use fast_input::FastInput;

fn main() {
    let inp = FastInput::new();
    inp.next_line(); 
    let buildings: Vec<i32> = inp.next_as_iter().collect();
    let mut i = 0;
    let mut best = 0;
    while i < buildings.len() {
        let mut j = i + 1;
        let mut lowest = buildings[i];

        while j < buildings.len() {
            lowest = lowest.min(buildings[j]);
            best = best.max(buildings[j].min(buildings[i]) - lowest);

            if buildings[j] >= buildings[i] {
                break;
            }

            j += 1;
        }

        i = j;
    }

    println!("{}", best);
}
