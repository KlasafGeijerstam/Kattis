mod fast_input;

use fast_input::{FastInput, FastParse};
use std::collections::BTreeSet;

struct Activity {
    start: u32,
    end: u32,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
struct Classroom {
    free_from: u32,
    id: usize,
}

fn main() {
    let inp = FastInput::new();
    let (activity_count, classroom_count): (usize, usize) = inp.next();
    let mut activities: Vec<_> = (0..activity_count)
        .map(|_| {
            let (start, end) = inp.next();

            Activity { start, end }
        })
        .collect();

    activities.sort_by_key(|k| k.end);

    let mut classrooms: BTreeSet<_> = (0..classroom_count).map(|id| Classroom { id, free_from: 0 }).collect();
    let mut activity_count = 0;
    for activity in activities {
        let needed_classroom = Classroom {
            free_from: activity.start,
            id: 0,
        };

        if let Some(mut room) = classrooms.range(..needed_classroom).last().copied() {
            classrooms.remove(&room);
            activity_count += 1;
            room.free_from = activity.end;
            classrooms.insert(room);
        }
    }

    println!("{}", activity_count);
}

