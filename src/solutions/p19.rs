pub fn s_v1() -> u64 {
    let mut months: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    let mut days: u16 = 0;
    for i in months.iter() {
        days += *i as u16;
    }

    let mut total: u64 = 0;
    for year in 1901..=2000 {
        months[1] = if is_leap_year(&year) { 29 } else { 28 };
        for month in 0..12 {
            if (days - 1) % 7 == 0 {
                total += 1;
            }
            days += months[month] as u16;
        }
    }

    total
}

fn is_leap_year(n: &u16) -> bool {
    if n % 4 == 0 {
        if n % 100 == 0 {
            return n % 400 == 0;
        }
        return true;
    }

    false
}

// Jan 31
// Feb 28 *29 on leap years (yyyy % 4 == 0) || (yyyy == nn00 % 400=== 0)
// Mar 31
// Apr 30
// May 31
// Jun 30
// Jul 31
// Aug 31
// Sep 30
// Oct 31
// Nov 30
// Dec 31
