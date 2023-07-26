use std::error::Error;
use std::num::ParseIntError;
use std::{env, process};

use chrono::Month::*;
use chrono::{Datelike, Duration, Month as ChronoMonth, NaiveDate, NaiveWeek, Weekday};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Week {
    start_date: NaiveDate,
    end_date: NaiveDate,
    start_day: Weekday,
}

#[derive(Debug, PartialEq, Clone)]
struct Month {
    name: ChronoMonth,
    weeks: Vec<Week>,
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
}

// This function defines the weeks between two dates.
// It doesn't account for the start / end of a year where the weeks usually
// overlap
fn define_week_range(start_date: NaiveDate, end_date: NaiveDate) -> Vec<Week> {
    let mut weeks: Vec<Week> = vec![];

    let dates: Vec<NaiveDate> = iter_days(start_date, end_date);

    for date in dates {
        if date.weekday() == Weekday::Mon {
            weeks.push(define_week(date));
        }
    }

    weeks
}

fn define_weeks_in_year(start_date: NaiveDate, end_date: NaiveDate) -> Vec<Week> {
    let first_week: Week = define_first_week(start_date);
    let last_week: Week = define_last_week(end_date);

    let first_day_normal_week: NaiveDate = add_day(first_week.end_date, 1);
    let last_day_normal_week: NaiveDate = sub_day(last_week.start_date, 1);

    let mut normal_week: Vec<Week> = define_week_range(first_day_normal_week, last_day_normal_week);
    normal_week.insert(0, first_week);
    normal_week.insert(normal_week.len(), last_week);

    normal_week
}

fn is_day_owned_by_month(day: NaiveDate, month: ChronoMonth) -> bool {
    day.month() == month.number_from_month()
}

fn how_many_days_in_week_owned_by_month(week: &Week, month: ChronoMonth) -> u8 {
    let complete_week = iter_days(week.start_date, week.end_date);

    let mut count: u8 = 0;

    for day in complete_week {
        if is_day_owned_by_month(day, month) {
            count += 1;
        }
    }

    count
}

fn is_week_owned_by_month(week: Week, month: ChronoMonth) -> bool {
    let first_date = week.start_date;
    if first_date.day() == 01 && first_date.month() == 1 && month.number_from_month() == 1 {
        return true;
    }

    let last_date = week.end_date;
    if last_date.day() == 31 && last_date.month() == 12 && month.number_from_month() == 12 {
        return true;
    }

    let number_of_days: u8 = how_many_days_in_week_owned_by_month(&week, month);

    match number_of_days {
        0..=3 => false,
        4..=u8::MAX => true,
    }
}

fn iter_days(start_date: NaiveDate, end_date: NaiveDate) -> Vec<NaiveDate> {
    if start_date > end_date {
        panic!("The start date must be leaser than the end date!");
    }

    start_date
        .iter_days()
        .take_while(|d| d != &add_day(end_date, 1))
        .collect::<Vec<NaiveDate>>()
}

fn split_in_month(weeks: Vec<Week>, month: ChronoMonth) -> Month {
    let weeks = weeks
        .into_iter()
        .filter(|week| is_week_owned_by_month(*week, month))
        .collect();

    Month { name: month, weeks }
}

fn split_in_months(weeks: Vec<Week>) -> Vec<Month> {
    let months: Vec<ChronoMonth> = vec![
        January, February, March, April, May, June, July, August, September, October, November,
        December,
    ];
    let mut res: Vec<_> = vec![];

    for month in months {
        let split_month = split_in_month(weeks.clone(), month);
        res.push(split_month);
    }

    res
}

fn display_year(year: i32, months: Vec<Month>) -> () {
    println!("{}", year);
    for month in months {
        println!("{:?} - {} weeks", month.name, month.weeks.len());
        display_weeks(month.weeks);
    }
}

fn display_weeks(weeks: Vec<Week>) -> () {
    for week in weeks {
        display_week(week);
    }
}

fn display_week(week: Week) -> () {
    println!(
        "{} - {}",
        week.start_date.format("%d").to_string(),
        week.end_date.format("%d").to_string()
    );
}

struct Config {
    year: i32,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let year = match args.next() {
            Some(year) => year,
            None => "no args found!".to_string(),
        };

        let year = Self::parse_argument(year).unwrap_or_else(|err| {
            println!("Problem parsing year argument: {}", err);
            process::exit(1);
        });

        Ok(Config { year })
    }

    fn parse_argument(arg: String) -> Result<i32, ParseIntError> {
        arg.parse()
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let fake_user_input_parsed: i32 = config.year;
    let first_year_day: NaiveDate = first_day_year(fake_user_input_parsed);

    let first_week: Week = define_first_week(first_year_day);
    let last_week: Week = define_last_week(last_day_year(fake_user_input_parsed));

    let year = define_weeks_in_year(first_week.start_date, last_week.end_date);

    let months: Vec<Month> = split_in_months(year);

    display_year(fake_user_input_parsed, months);

    Ok(())
}

fn main() {
    let args = env::args();
    println!("{:?}", args);

    let config: Config = Config::build(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let carret = "-".repeat(6);
    println!("{} Budget Planner! {}", carret, carret);

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}

#[cfg(test)]
mod test {
    use crate::{is_week_owned_by_month, *};

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

        let defined_week: Week = define_week(start_date);

        assert_eq!(defined_week, week);
        assert_eq!(defined_week.start_day, Weekday::Mon);
    }

    #[test]
    fn define_range_week_special_start() {
        let start_date = NaiveDate::from_ymd_opt(2023, 01, 02).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();

        let year: Vec<Week> = define_week_range(start_date, end_date);

        assert_eq!(year.len(), 52);

        let first_date: &NaiveDate = &year.first().unwrap().start_date;
        let last_date: &NaiveDate = &year.last().unwrap().end_date;

        assert_eq!(&start_date, first_date);
        assert_eq!(&end_date, last_date);
    }

    #[test]
    fn define_range_week_special_end() {
        let start_date = NaiveDate::from_ymd_opt(2024, 01, 01).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 29).unwrap();

        let year: Vec<Week> = define_week_range(start_date, end_date);

        assert_eq!(year.len(), 52);

        let first_date: &NaiveDate = &year.first().unwrap().start_date;
        let last_date: &NaiveDate = &year.last().unwrap().end_date;

        assert_eq!(&start_date, first_date);
        assert_eq!(&end_date, last_date);
    }

    #[test]
    fn define_range_week_special_start_and_end() {
        let start_date = NaiveDate::from_ymd_opt(2022, 01, 03).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2022, 12, 25).unwrap();

        let year: Vec<Week> = define_week_range(start_date, end_date);

        assert_eq!(year.len(), 51);

        let first_date: &NaiveDate = &year.first().unwrap().start_date;
        let last_date: &NaiveDate = &year.last().unwrap().end_date;

        assert_eq!(&start_date, first_date);
        assert_eq!(&end_date, last_date);
    }

    #[test]
    fn iter_over_day() {
        let start_date = NaiveDate::from_ymd_opt(2023, 01, 01).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2023, 01, 06).unwrap();

        let result: Vec<NaiveDate> = iter_days(start_date, end_date);

        assert_eq!(result.len(), 6);

        let first_date: &NaiveDate = result.first().unwrap();
        let last_date: &NaiveDate = result.last().unwrap();

        assert_eq!(&start_date, first_date);
        assert_eq!(&end_date, last_date);
    }

    #[test]
    fn iter_over_day_on_full_year() {
        let start_date = NaiveDate::from_ymd_opt(2023, 01, 01).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();

        let result: Vec<NaiveDate> = iter_days(start_date, end_date);

        assert_eq!(result.len(), 365);

        let first_date: &NaiveDate = result.first().unwrap();
        let last_date: &NaiveDate = result.last().unwrap();

        assert_eq!(&start_date, first_date);
        assert_eq!(&end_date, last_date);
    }

    #[test]
    #[should_panic]
    fn iter_over_days_reverse() {
        let start_date = NaiveDate::from_ymd_opt(2023, 01, 01).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2023, 01, 06).unwrap();

        iter_days(end_date, start_date);
    }

    #[test]
    fn define_weeks_year() {
        let start_date = NaiveDate::from_ymd_opt(2023, 01, 01).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();

        let year: Vec<Week> = define_weeks_in_year(start_date, end_date);

        assert_eq!(year.len(), 53);

        let first_date: &NaiveDate = &year.first().unwrap().start_date;
        let last_date: &NaiveDate = &year.last().unwrap().end_date;

        assert_eq!(&start_date, first_date);
        assert_eq!(&end_date, last_date);
    }

    #[test]
    fn define_weeks_year_leap() {
        let start_date = NaiveDate::from_ymd_opt(2024, 01, 01).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

        let year: Vec<Week> = define_weeks_in_year(start_date, end_date);

        assert_eq!(year.len(), 53);

        let first_date: &NaiveDate = &year.first().unwrap().start_date;
        let last_date: &NaiveDate = &year.last().unwrap().end_date;

        assert_eq!(&start_date, first_date);
        assert_eq!(&end_date, last_date);
    }

    #[test]
    fn is_day_owned_by_month_positive() {
        let date = NaiveDate::from_ymd_opt(2022, 03, 28).unwrap();
        let month = ChronoMonth::March;

        let result: bool = is_day_owned_by_month(date, month);

        assert!(result);
    }

    #[test]
    fn is_day_owned_by_month_negative() {
        let date = NaiveDate::from_ymd_opt(2022, 04, 28).unwrap();
        let month = ChronoMonth::March;

        let result: bool = is_day_owned_by_month(date, month);

        assert!(!result);
    }

    #[test]
    fn how_many_days_week_owned_by_month_four_days() {
        let start_date = NaiveDate::from_ymd_opt(2022, 03, 28).unwrap();
        let week: Week = define_week(start_date);
        let month = ChronoMonth::March;

        let result: u8 = how_many_days_in_week_owned_by_month(&week, month);

        assert_eq!(result, 4);
    }

    #[test]
    fn how_many_days_week_owned_by_month_three_days() {
        let start_date = NaiveDate::from_ymd_opt(2022, 03, 28).unwrap();
        let week: Week = define_week(start_date);
        let month = ChronoMonth::April;

        let result: u8 = how_many_days_in_week_owned_by_month(&week, month);

        assert_eq!(result, 3);
    }

    #[test]
    fn how_many_days_week_owned_by_month_seven_days() {
        let start_date = NaiveDate::from_ymd_opt(2022, 04, 18).unwrap();
        let week: Week = define_week(start_date);
        let month = ChronoMonth::April;

        let result: u8 = how_many_days_in_week_owned_by_month(&week, month);

        assert_eq!(result, 7);
    }

    #[test]
    fn is_week_owned_by_month_three_days() {
        let start_date = NaiveDate::from_ymd_opt(2022, 03, 28).unwrap();
        let week: Week = define_week(start_date);
        let month = ChronoMonth::April;

        let result = is_week_owned_by_month(week, month);

        assert!(!result);
    }

    #[test]
    fn is_week_owned_by_month_four_days() {
        let start_date = NaiveDate::from_ymd_opt(2022, 03, 28).unwrap();
        let week: Week = define_week(start_date);
        let month = ChronoMonth::March;

        let result = is_week_owned_by_month(week, month);

        assert!(result);
    }

    #[test]
    fn is_week_owned_by_month_seven_days() {
        let start_date = NaiveDate::from_ymd_opt(2022, 04, 18).unwrap();
        let week: Week = define_week(start_date);
        let month = ChronoMonth::April;

        let result = is_week_owned_by_month(week, month);

        assert!(result);
    }

    #[test]
    fn is_week_owned_by_month_first_week_in_year() {
        let start_date = NaiveDate::from_ymd_opt(2023, 01, 01).unwrap();
        let week: Week = define_first_week(start_date);
        let month = ChronoMonth::January;

        let result = is_week_owned_by_month(week, month);

        assert!(result);
    }

    #[test]
    fn is_week_owned_by_month_last_week_in_year() {
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let week: Week = define_last_week(end_date);
        let month = ChronoMonth::December;

        let result = is_week_owned_by_month(week, month);

        assert!(result);
    }

    #[test]
    fn split_month_into_budget_weeks_four_weeks() {
        let start_date = NaiveDate::from_ymd_opt(2024, 01, 01).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let month = January;

        let weeks: Vec<Week> = define_weeks_in_year(start_date, end_date);

        let result: Month = split_in_month(weeks, month);

        assert_eq!(result.weeks.len(), 4);
    }

    #[test]
    fn split_month_into_budget_weeks_five_weeks() {
        let start_date = NaiveDate::from_ymd_opt(2024, 01, 01).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        let month = February;

        let weeks: Vec<Week> = define_weeks_in_year(start_date, end_date);

        let result = split_in_month(weeks, month);

        assert_eq!(result.weeks.len(), 5);
    }

    #[test]
    fn split_months_into_budget_weeks_1() {
        let start_date = NaiveDate::from_ymd_opt(2024, 01, 01).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

        let weeks: Vec<Week> = define_weeks_in_year(start_date, end_date);

        let result: Vec<Month> = split_in_months(weeks);

        assert_eq!(result.len(), 12);
        assert_eq!(
            result
                .clone()
                .into_iter()
                .map(|month| month.weeks.len())
                .collect::<Vec<usize>>(),
            vec![4, 5, 4, 4, 5, 4, 4, 5, 4, 5, 4, 5]
        );
        assert_eq!(
            result
                .into_iter()
                .map(|month| month.weeks.len())
                .sum::<usize>(),
            53
        );
    }

    #[test]
    fn split_months_into_budget_weeks_2() {
        let start_date = NaiveDate::from_ymd_opt(2023, 01, 01).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2023, 12, 31).unwrap();

        let weeks: Vec<Week> = define_weeks_in_year(start_date, end_date);

        let result: Vec<Month> = split_in_months(weeks);

        assert_eq!(result.len(), 12);
        assert_eq!(
            result
                .clone()
                .into_iter()
                .map(|month| month.weeks.len())
                .collect::<Vec<usize>>(),
            vec![5, 4, 5, 4, 4, 5, 4, 5, 4, 4, 5, 4]
        );
        assert_eq!(
            result
                .into_iter()
                .map(|month| month.weeks.len())
                .sum::<usize>(),
            53
        );
    }
}
