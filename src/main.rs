use chrono::{Datelike, Duration, NaiveDate, NaiveWeek, Weekday};

#[derive(Debug, PartialEq)]
struct Week {
    start_date: NaiveDate,
    end_date: NaiveDate,
    start_day: Weekday,
    // days: Vec<NaiveDate>,
}

struct WeekOther {
    start_day: Weekday,
    days: Vec<NaiveDate>,
}

fn first_day_year(year: i32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, 01, 01).unwrap()
}

fn last_day_year(year: i32) -> NaiveDate {
    NaiveDate::from_ymd_opt(year, 12, 31).unwrap()
}

fn add_day(date: NaiveDate, nbr: i64) -> NaiveDate {
    date + Duration::days(nbr)
}

fn sub_day(date: NaiveDate, nbr: i64) -> NaiveDate {
    date - Duration::days(nbr)
}

fn define_first_week(date: NaiveDate) -> Week {
    let weekday: Weekday = date.weekday();
    let week = date.week(weekday);

    match weekday {
        Weekday::Mon => Week {
            start_date: week.first_day(),
            start_day: date.weekday(),
            end_date: week.last_day(),
        },
        Weekday::Tue => Week {
            start_date: week.first_day(),
            start_day: date.weekday(),
            end_date: add_day(week.first_day(), 5),
        },
        Weekday::Wed => Week {
            start_date: week.first_day(),
            start_day: date.weekday(),
            end_date: add_day(week.first_day(), 4),
        },
        Weekday::Thu => Week {
            start_date: week.first_day(),
            start_day: date.weekday(),
            end_date: add_day(week.first_day(), 3),
        },
        Weekday::Fri => Week {
            start_date: week.first_day(),
            start_day: date.weekday(),
            end_date: add_day(week.first_day(), 2),
        },
        Weekday::Sat => Week {
            start_date: week.first_day(),
            start_day: date.weekday(),
            end_date: add_day(week.first_day(), 1),
        },
        Weekday::Sun => Week {
            start_date: week.first_day(),
            start_day: date.weekday(),
            end_date: week.first_day(),
        },
    }
}

/// We want to define the last week of a year based on the last day.
/// We don't want to have the last week overlap on the next year
fn define_last_week(date: NaiveDate) -> Week {
    let weekday: Weekday = date.weekday();

    match weekday {
        Weekday::Sun => {
            let start_date: NaiveDate = sub_day(date, 6);
            Week {
                start_date,
                start_day: start_date.weekday(),
                end_date: date,
            }
        }
        Weekday::Sat => {
            let start_date: NaiveDate = sub_day(date, 5);
            Week {
                start_date,
                start_day: start_date.weekday(),
                end_date: date,
            }
        }
        Weekday::Fri => {
            let start_date: NaiveDate = sub_day(date, 4);
            Week {
                start_date,
                start_day: start_date.weekday(),
                end_date: date,
            }
        }
        Weekday::Thu => {
            let start_date: NaiveDate = sub_day(date, 3);
            Week {
                start_date,
                start_day: start_date.weekday(),
                end_date: date,
            }
        }
        Weekday::Wed => {
            let start_date: NaiveDate = sub_day(date, 2);
            Week {
                start_date,
                start_day: start_date.weekday(),
                end_date: date,
            }
        }
        Weekday::Tue => {
            let start_date: NaiveDate = sub_day(date, 1);
            Week {
                start_date,
                start_day: start_date.weekday(),
                end_date: date,
            }
        }
        Weekday::Mon => Week {
            start_date: date,
            start_day: date.weekday(),
            end_date: date,
        },
    }
}

// Those week should always start with a monday and end with a sunday
// Because we already covered the two special cases.
fn define_week(date: NaiveDate) -> Week {
    let start_day: Weekday = date.weekday();
    let week: NaiveWeek = date.week(start_day);
    let start_date: NaiveDate = week.first_day();
    let end_date: NaiveDate = week.last_day();

    Week {
        start_date,
        end_date,
        start_day,
    }
    /* let weekday: Weekday = date.weekday();
    let days = date
        .iter_days()
        .take_while(|d| d.year() == year)
        .enumerate(); */

    /* for day in days {
        println!("{:?}", day);
        WeekOther {
            start_day: date.weekday(),
            days: vec!(),
        }
    } */
}

fn define_week_range(start_date: NaiveDate, end_date: NaiveDate) -> Vec<Week> {
    let mut weeks: Vec<Week> = vec![];

    let dates: Vec<NaiveDate> = start_date.iter_weeks().take_while(|d| d != &end_date).collect();

    for date in dates {
        weeks.push(define_week(date));
    }

    weeks
}

fn main() {
    let carret = "-".repeat(6);
    println!("{} Budget Planner! {}", carret, carret);

    let _fake_user_input = String::from("2022");
    let fake_user_input_parsed: i32 = 2022;
    let first_year_day: NaiveDate = first_day_year(fake_user_input_parsed);

    let first_week: Week = define_first_week(first_year_day);
    let last_week: Week = define_last_week(last_day_year(fake_user_input_parsed));

    let first_day_normal_week: NaiveDate = add_day(first_week.end_date, 1);
    let last_day_normal_week: NaiveDate = last_week.start_date;
    println!("{:?}", first_day_normal_week);
    println!("{:?}", last_day_normal_week);

    let mut normal_weeks: Vec<Week> = define_week_range(first_day_normal_week, last_day_normal_week);

    normal_weeks.insert(0, first_week);
    normal_weeks.append(&mut vec![last_week]);

    for week in normal_weeks {
        println!("{:?}", week);
    }
    /* let next_day = first_week.end_date + Duration::days(1);

    weeks.push(first_week);

    for week in next_day
        .iter_weeks()
        .take_while(|w| w.year() == fake_user_input_parsed)
    {
        let test = week.week(Weekday::Mon);
        let test2 = Week {
            start_date: test.first_day(),
            start_day: week.weekday(),
            end_date: test.last_day(),
        };

        weeks.push(test2);
    }

    for w in &weeks {
        println!("{:?}", w);
    }

    println!("{}", carret);

    define_week(first_year_day); */

    // let mut last_week: &Week = &weeks[weeks.len() - 1];
    // last_week.end_date = last_week.end_date - 1;
    // println!("{:?}", last_week);
}

#[cfg(test)]
mod test {
    use crate::*;
    use chrono::NaiveDate;

    #[test]
    fn first_day_year_2022() {
        let first_day_2022: NaiveDate = NaiveDate::from_ymd_opt(2022, 01, 01).unwrap();
        assert_eq!(first_day_year(2022), first_day_2022);
    }

    #[test]
    fn last_day_year_2022() {
        let last_day_2022: NaiveDate = NaiveDate::from_ymd_opt(2022, 12, 31).unwrap();
        assert_eq!(last_day_year(2022), last_day_2022);
    }

    #[test]
    fn first_day_year_2023() {
        let first_day_2023: NaiveDate = NaiveDate::from_ymd_opt(2023, 01, 01).unwrap();
        assert_eq!(first_day_year(2023), first_day_2023);
    }

    #[test]
    fn last_day_year_2023() {
        let last_day_2023: NaiveDate = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();
        assert_eq!(last_day_year(2023), last_day_2023);
    }

    #[test]
    fn define_week_starting_with_monday() {
        let start_date = NaiveDate::from_ymd_opt(2024, 01, 01).unwrap();
        let week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date: NaiveDate::from_ymd_opt(2024, 01, 07).unwrap(),
        };

        assert_eq!(define_first_week(start_date), week);
    }

    #[test]
    fn define_week_starting_with_tuesday() {
        let start_date: NaiveDate = NaiveDate::from_ymd_opt(2024, 01, 02).unwrap();
        let week: Week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date: NaiveDate::from_ymd_opt(2024, 01, 07).unwrap(),
        };

        assert_eq!(define_first_week(start_date), week);
    }

    #[test]
    fn define_week_starting_with_wednesday() {
        let start_date: NaiveDate = NaiveDate::from_ymd_opt(2024, 01, 03).unwrap();
        let week: Week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date: NaiveDate::from_ymd_opt(2024, 01, 07).unwrap(),
        };

        assert_eq!(define_first_week(start_date), week);
    }

    #[test]
    fn define_week_starting_with_thursday() {
        let start_date: NaiveDate = NaiveDate::from_ymd_opt(2024, 01, 04).unwrap();
        let week: Week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date: NaiveDate::from_ymd_opt(2024, 01, 07).unwrap(),
        };

        assert_eq!(define_first_week(start_date), week);
    }

    #[test]
    fn define_week_starting_with_friday() {
        let start_date: NaiveDate = NaiveDate::from_ymd_opt(2024, 01, 05).unwrap();
        let week: Week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date: NaiveDate::from_ymd_opt(2024, 01, 07).unwrap(),
        };

        assert_eq!(define_first_week(start_date), week);
    }

    #[test]
    fn define_week_starting_with_saturday() {
        let start_date = NaiveDate::from_ymd_opt(2022, 01, 01).unwrap();
        let week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date: NaiveDate::from_ymd_opt(2022, 01, 02).unwrap(),
        };
        assert_eq!(define_first_week(start_date), week);
    }

    #[test]
    fn define_week_starting_with_sunday() {
        let start_date = NaiveDate::from_ymd_opt(2023, 01, 01).unwrap();
        let week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date: NaiveDate::from_ymd_opt(2023, 01, 01).unwrap(),
        };

        assert_eq!(define_first_week(start_date), week);
    }

    #[test]
    fn define_week_ending_with_sunday() {
        let end_date = NaiveDate::from_ymd_opt(2023, 05, 28).unwrap();
        let start_date = NaiveDate::from_ymd_opt(2023, 05, 22).unwrap();

        let week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date,
        };

        assert_eq!(define_last_week(end_date), week);
    }

    #[test]
    fn define_week_ending_with_sartuday() {
        let end_date = NaiveDate::from_ymd_opt(2023, 05, 27).unwrap();
        let start_date = NaiveDate::from_ymd_opt(2023, 05, 22).unwrap();

        let week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date,
        };

        assert_eq!(define_last_week(end_date), week);
    }

    #[test]
    fn define_week_ending_with_friday() {
        let end_date = NaiveDate::from_ymd_opt(2023, 05, 26).unwrap();
        let start_date = NaiveDate::from_ymd_opt(2023, 05, 22).unwrap();

        let week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date,
        };

        assert_eq!(define_last_week(end_date), week);
    }

    #[test]
    fn define_week_ending_with_thursday() {
        let end_date = NaiveDate::from_ymd_opt(2023, 05, 25).unwrap();
        let start_date = NaiveDate::from_ymd_opt(2023, 05, 22).unwrap();

        let week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date,
        };

        assert_eq!(define_last_week(end_date), week);
    }

    #[test]
    fn define_week_ending_with_wednesday() {
        let end_date = NaiveDate::from_ymd_opt(2023, 05, 24).unwrap();
        let start_date = NaiveDate::from_ymd_opt(2023, 05, 22).unwrap();

        let week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date,
        };

        assert_eq!(define_last_week(end_date), week);
    }

    #[test]
    fn define_week_ending_with_tuesday() {
        let end_date = NaiveDate::from_ymd_opt(2023, 05, 23).unwrap();
        let start_date = NaiveDate::from_ymd_opt(2023, 05, 22).unwrap();

        let week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date,
        };

        assert_eq!(define_last_week(end_date), week);
    }

    #[test]
    fn define_week_ending_with_monday() {
        let end_date = NaiveDate::from_ymd_opt(2023, 05, 22).unwrap();
        let start_date = NaiveDate::from_ymd_opt(2023, 05, 22).unwrap();

        let week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date,
        };

        assert_eq!(define_last_week(end_date), week);
    }

    #[test]
    fn define_normal_week() {
        let start_date = NaiveDate::from_ymd_opt(2023, 05, 01).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2023, 05, 07).unwrap();

        let week = Week {
            start_date,
            start_day: start_date.weekday(),
            end_date,
        };

        assert_eq!(define_week(start_date), week);
    }
}
