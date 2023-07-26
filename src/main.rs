use std::process;

use budget_planner::Config;
use clap::Parser;

fn main() {
    let config = Config::parse();

    let carret = "-".repeat(6);
    println!("{} Budget Planner! {}", carret, carret);

    if let Err(e) = budget_planner::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
