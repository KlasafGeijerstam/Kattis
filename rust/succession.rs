mod fast_input;
use fast_input::{Str, FastInput};
use std::collections::HashMap;

struct Person<'a> {
    blood: Option<u64>,
    p1: &'a str,
    p2: &'a str
}

fn get_blood<'a>(people: &mut HashMap<&'a str, Person<'a>>, person: &'a str) -> u64 {
    if !people.contains_key(person) {
        people.insert(person, Person {
            blood: Some(0),
            p1: "",
            p2: "",
        });
    }

    if let Some(blood) = people[person].blood {
        blood
    } else {
        let p1_blood = get_blood(people, people[person].p1);
        let p2_blood = get_blood(people, people[person].p2);
        let person = people.get_mut(person).unwrap();
        person.blood = Some(p1_blood / 2 + p2_blood  / 2);

        person.blood.unwrap()
    }
}

fn main() {
    let inp = FastInput::new();
    let (n, m) = inp.next_tuple();
    let founder = inp.next_line();
    let mut people = HashMap::new();
    people.insert(founder, Person {
        blood: Some(2u64.pow(63)),
        p1: "",
        p2: ""
    });

    for _ in 0..n {
        let (child, p1, p2): (Str, Str, Str) = inp.next_triple();
        people.insert(*child, Person {
            blood: None,
            p1: *p1,
            p2: *p2
        });
    }
    
    let (_, noble) = (0..m).map(|_| {
        let claimant = inp.next_line();
        (get_blood(&mut people, claimant), claimant)
    }).max().unwrap();
    println!("{}", noble);
}
