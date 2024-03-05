use std::collections::HashMap;
fn in_leap_year(year: i32) -> bool {
    // 4000 -> no leap year
    if year == 4000 {
        return false;
    }
    // modulo 400 always leap year
    if year % 400 == 0 {
        return true;
    }
    // modulo 4 but not modulo 100
    if year % 4 == 0 && year % 100 != 0 {
        return true;
    }
    false
}

#[derive(Copy, Clone)]
struct Date {
    year: i32,
    month: i32,
    day: i32,
}

fn calculate_appended_date(now: Date, remaining_days: i32, map: &HashMap<i32, i32>) -> Date {
    let mut r_days = remaining_days;
    let mut days_in_month = map.get(&now.month).unwrap_or(&0);
    let mut date = Date {
        year: now.year,
        month: now.month,
        day: now.day,
    };
    while r_days > *days_in_month {
        days_in_month = map.get(&date.month).unwrap_or(&0);
        if date.month == 12 {
            date.year += 1;
            date.month = 1;
        }
        if in_leap_year(date.year) && date.month == 2 {
            r_days -= 29;
        } else {
            r_days -= *days_in_month;
        }
        date.month += 1;
    }
    date.day += r_days;
    date
}

pub fn year_calc() {
    let mut line = String::new();
    let mut line2 = String::new();
    println!("Enter year :");
    std::io::stdin().read_line(&mut line).unwrap();
    println!("line: {}", line);
    let year = line.trim().parse::<i32>().unwrap();
    println!("Leap year? {}", in_leap_year(year));

    println!("It is now 2024.01.01. Please add a number of days to it. Day: ");
    std::io::stdin().read_line(&mut line2).unwrap();
    println!("line: {}", line2);
    let day = line2.trim().parse::<i32>().unwrap();

    let s = HashMap::from([
        (1, 31),
        (2, 28),
        (3, 31),
        (4, 30),
        (5, 31),
        (6, 30),
        (7, 31),
        (8, 31),
        (9, 30),
        (10, 31),
        (11, 30),
        (12, 31),
    ]);

    let now = Date {
        year: 2024,
        month: 1,
        day: 1,
    };
    let date = calculate_appended_date(now, day, &s);
    println!("date: {}.{}.{}", date.year, date.month, date.day);
}
