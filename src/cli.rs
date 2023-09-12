use chrono::NaiveDate;
use clap::Parser;
use std::error::Error;

use crate::golden_time::{
    define_first_week, define_last_week, define_weeks_in_year, display_year, first_day_year,
    last_day_year, split_in_months, Month, Week,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    pub year: i32,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let input_parsed: i32 = config.year;
    let first_year_day: NaiveDate = first_day_year(input_parsed);

    let first_week: Week = define_first_week(first_year_day);
    let last_week: Week = define_last_week(last_day_year(input_parsed));

    let year = define_weeks_in_year(first_week.start_date, last_week.end_date);

    let months: Vec<Month> = split_in_months(year);

    display_year(input_parsed, months);

    Ok(())
}
