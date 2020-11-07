mod fast_input;
use fast_input::FastInput;

mod point;
use point::Point;

mod convex_hull;
use convex_hull::convex_hull;

fn main() {
    let inp = FastInput::new();
    while inp.has_next_line() {
        let nums: Vec<f64> = inp.next_as_iter().collect();
        let points: Vec<_> = (0..nums.len())
            .step_by(2)
            .map(|i| Point::new(nums[i], nums[i + 1]))
            .collect();
        let n = points.len() as f64;
        let hull = convex_hull(points);
        let mut s = 0.0;
        for (i, p) in hull.iter().enumerate() {
            s += p.dist_between(&hull[(i + 1) % hull.len()]);
        }
        println!("{}", 100.0 * n / (1.0 + s));
    }
}
