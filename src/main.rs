use std::process;

use budget_planner::cli::Config;
use budget_planner::cli::run;
use clap::Parser;

fn main() {
    let config = Config::parse();

    let carret = "-".repeat(6);
    println!("{} Budget Planner! {}", carret, carret);

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}
