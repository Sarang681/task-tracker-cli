use clap::Parser;
use task_tracker::{Arg, Commands};

fn main() {
    let args = Arg::parse();

    if let Commands::Add { task } = args.cmd {
        println!("Task is :: {}", task);
    } else {
        println!("{:?}", args);
    };
}
