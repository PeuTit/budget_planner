use std::{process, env};

use budget_planner::Config;

fn main() {
    let args = env::args();
    println!("{:?}", args);

    let config: Config = Config::build(args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let carret = "-".repeat(6);
    println!("{} Budget Planner! {}", carret, carret);

    if let Err(e) = budget_planner::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    };
}
