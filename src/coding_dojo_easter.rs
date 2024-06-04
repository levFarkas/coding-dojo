use chrono::{Datelike, Duration, NaiveDate, Weekday};
use solar_calendar_events::MarchEquinox;

pub fn get_easter_sunday() {
    let mut line = String::new();
    println!("Enter year :");
    std::io::stdin().read_line(&mut line).unwrap();
    let year = line.trim().parse::<i32>().unwrap();

    if year < 1900 || year > 2099 {
        println!("Not supported year :(");
        return;
    }

    // Calculate D = 225 - 11(Y MOD 19).
    let mut d = 225 - 11 * (year % 19);

    // If D is greater than 50 then subtract multiples of 30 until the resulting new value of D is less than 51.
    if d > 50 {
        while d > 50 {
            d = d - 30
        }
    }
    // If D is greater than 48 subtract 1 from it.
    if d > 48 {
        d = d - 1
    }

    // Calculate E = (Y + [Y/4] + D + 1) MOD 7. (NB Integer part of [Y/4])
    let e = (year + year / 4 + d + 1) % 7;

    // Calculate Q = D + 7 - E.
    let q = d + 7 - e;

    // If Q is less than 32 then Easter is in March. If Q is greater than 31 then Q - 31 is its date in April.
    if q > 31 {
        println!("04.{}", q - 31);
    } else {
        println!("03.{}", q);
    }

    // Calculate equinox + full moon

    let march_equinox = MarchEquinox::new(year);
    let equinox_date_time = march_equinox.unwrap().date_time().unwrap().date();

    // TODO get next full moon
    let month = equinox_date_time.month().to_string();

    let mut month_number = 3;
    if month == "April" {
        month_number = 4;
    }
    get_next_sunday(
        NaiveDate::from_ymd_opt(
            equinox_date_time.year(),
            month_number,
            u32::from(equinox_date_time.day()),
        )
        .unwrap(),
    )
}

fn get_next_sunday(date: NaiveDate) {
    // Get the current weekday (0 = Sunday, 6 = Saturday)
    for day in 0..7 {
        println!(
            "{}",
            (date + Duration::days(7 - date.weekday().num_days_from_monday() as i64 + day))
                .format("%Y-%m-%d %A")
        );
        println!("=======================");
        if (date + Duration::days(7 - date.weekday().num_days_from_monday() as i64 + day)).weekday()
            == Weekday::Sun
        {
            println!("Ez is husvet!");
        }
    }
}
