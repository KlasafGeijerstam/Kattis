mod inp;
use inp::Inp;

use std::io::Result;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

fn haversine(lat1: f32, long1: f32, lat2: f32, long2: f32) -> f32 {
    f32::powf(f32::sin((lat2 - lat1) / 2.0), 2.0) + f32::cos(lat1)*f32::cos(lat2)*f32::powf(f32::sin((long2 - long1) / 2.0), 2.0)
}

fn c((lat1, long1): (f32, f32), (lat2, long2): (f32, f32)) -> f32 {
    2.0 * f32::atan2(f32::sqrt(haversine(lat1, long1, lat2, long2)), f32::sqrt(1.0 - haversine(lat1, long1, lat2, long2)))
}

fn lat_dist(p1: (f32, f32), p2: (f32, f32)) -> f32 {
    return 6381.0 * c(p1, p2)
}

#[derive(PartialEq)]
struct MF(pub f32);

impl Eq for MF {}

impl PartialOrd for MF {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.0.partial_cmp(&other.0)
    }
}

impl Ord for MF {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}


fn main() -> Result<()> {
    let mut inp = Inp::new();
    let (n, m) = inp.next_tuple::<usize>();
    let (start, end) = inp.next_str_tuple();
    let start = start.to_owned();
    let end = end.to_owned();

    let mut name_index = HashMap::new();
    name_index.insert(start.clone(), 0);
    name_index.insert(end.clone(), 1);
    let mut airports = Vec::with_capacity(n);
    airports.push((0.0f32, 0.0f32));
    airports.push((0.0, 0.0));
    let mut connections = vec![vec![]; n];

    for _ in 0..n {
        let (name, lat, lng) = inp.next_str_triple();
        let pos = (lat.parse::<f32>().unwrap().to_radians(),
                   lng.parse::<f32>().unwrap().to_radians());
        if name == &start {
            airports[0] = pos;
        } else if name == &end {
            airports[1] = pos;
        } else {
            name_index.insert(name.to_owned(), airports.len());
            airports.push(pos);
        }
    }
    for _ in 0..m {
        let (from, to) = inp.next_str_tuple(); 
        let (from, to) = (name_index[from], name_index[to]);
        let dist = 100.0 + lat_dist(airports[from], airports[to]);
        connections[from].push((to, dist));
        connections[to].push((from, dist));
    }
    
    
    let mut dist = vec![std::f32::INFINITY; n];
    dist[0] = 0.0;
    let mut pq = BinaryHeap::new();
    pq.push((MF(0.0), 0));
    while !pq.is_empty() {
        let u = pq.pop().unwrap().1;
        for (v, w) in &connections[u] {
            if dist[*v] > dist[u] + w {
                dist[*v] = dist[u] + w;
                pq.push((MF(dist[*v]), *v));
            }
        }
    }

    if dist[1] == std::f32::INFINITY {
        println!("-1");
    } else {
        println!("{}", dist[1]);
    }
    Ok(())
}
