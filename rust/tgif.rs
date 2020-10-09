mod inp;
use inp::Inp;
use std::collections::HashSet;

fn main() {
    let mut inp = Inp::new();
    let (today, month) = {
        let mut l = inp.next_line().trim().split(' ');
        (l.next().unwrap().parse::<u16>().unwrap(),
            match l.next().unwrap() {
                "JAN" => 0,    
                "FEB" => 1,    
                "MAR" => 2,    
                "APR" => 3,    
                "MAY" => 4,    
                "JUN" => 5,    
                "JUL" => 6,    
                "AUG" => 7,    
                "SEP" => 8,    
                "OCT" => 9,    
                "NOV" => 10,    
                _ => 11,    
            })
    };
    let mut day = {
        match inp.next_line().trim() {
            "MON" => 0,
            "TUE" => 1,
            "WED" => 2,
            "THU" => 3,
            "FRI" => 4,
            "SAT" => 5,
            _ => 6,
        }
    };
    

    let mut days = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    if today == 29 && month == 1 {
        days[1] = 29;
    }
    let mut cmonth = 0; 
    let mut cday = 1;
    while cmonth != month || cday != today {
        if cday == days[cmonth] {
            cmonth += 1;
            cday = 1;
        } else {
            cday += 1;
        }
        day = (day + 1) % 7;
    }
    if month <= 1 && day == 4 {
        println!("TGIF");
    } else if month <= 1 && day != 4{
        println!(":(");
    } else if day == 3 || day == 4 {
        println!("not sure");
    } else {
        println!(":(");
    }
}
