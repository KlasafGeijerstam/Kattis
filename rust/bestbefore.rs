mod fast_input;

use fast_input::FastInput;

fn is_leap(year: u16) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn days_in_month(year: u16, month: u16) -> u16 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        2 => {
            if is_leap(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

fn main() {
    let inp = FastInput::new();
    let input = inp.next_line();
    let parts: Vec<u16> = input.split('/').map(|p| p.parse().unwrap()).collect();
    let min_date = [
        (0, 1, 2),
        (0, 2, 1),
        (1, 2, 0),
        (1, 0, 2),
        (2, 1, 0),
        (2, 0, 1),
    ]
    .iter()
    .filter_map(|&(y1, y2, y3)| {
        let mut year = parts[y1];
        let month = parts[y2];
        let day = parts[y3];
        if year < 2000 {
            year += 2000;
        }
        if month > 0 && month <= 12 && day > 0 && day <= days_in_month(year, month) {
            Some((year, month, day))
        } else {
            None
        }
    })
    .min();

    if let Some((year, month, day)) = min_date {
        println!("{}-{:02}-{:02}", year, month, day);
    } else {
        println!("{} is illegal", input);
    }
}
