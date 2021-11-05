mod fast_input;
use std::collections::HashMap;
use std::io::{stdout, BufWriter, Write};

use fast_input::{FastInput, Str};

fn main() {
    let inp = FastInput::new();
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for _ in 0..inp.next() {
        let (n, m) = inp.next_tuple();
        let cars: HashMap<&'_ str, Car> = (0..n)
            .map(|_| {
                let (name, catalog_price, pickup_price, kilometer_price): (Str, _, _, _) =
                    inp.next_quad();
                (
                    *name,
                    Car {
                        catalog_price,
                        pickup_price,
                        kilometer_price,
                    },
                )
            })
            .collect();

        let mut agents = HashMap::new();

        for _ in 0..m {
            let (_, agent_name, action, k): (u32, Str, char, Str) = inp.next_quad();

            let agent = agents.entry(*agent_name).or_insert(Agent {
                cost: 0,
                car: None,
                consistent: true,
            });

            if !agent.consistent {
                continue;
            }

            match action {
                'p' => {
                    if agent.car.is_none() {
                        agent.car = Some(*k);
                        agent.cost += cars[*k].pickup_price;
                    } else {
                        agent.consistent = false;
                    }
                }
                'r' => {
                    if let Some(car_model) = agent.car {
                        let distance: u32 = k.parse().unwrap();
                        agent.cost += cars[car_model].kilometer_price * distance;
                        agent.car = None;
                    } else {
                        agent.consistent = false;
                    }
                }
                _ => {
                    if let Some(car_model) = agent.car {
                        let damage: u32 = k.parse().unwrap();
                        let car_cost = cars[car_model].catalog_price;
                        agent.cost += if (damage * car_cost) % 100 == 0 {
                            (damage * car_cost) / 100
                        } else {
                            (damage * car_cost) / 100 + 1
                        }
                    } else {
                        agent.consistent = false;
                    }
                }
            }
        }

        let mut agent_names: Vec<_> = agents.keys().collect();
        agent_names.sort_unstable();

        for &name in agent_names {
            let agent = &agents[name];
            
            if agent.consistent && agent.car.is_none() {
                writeln!(writer, "{} {}", name, agent.cost).ok();
            } else {
                writeln!(writer, "{} INCONSISTENT", name).ok();
            }
        }
    }
}

struct Car {
    catalog_price: u32,
    pickup_price: u32,
    kilometer_price: u32,
}

struct Agent<'a> {
    cost: u32,
    car: Option<&'a str>,
    consistent: bool,
}

