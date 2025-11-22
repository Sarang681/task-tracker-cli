use clap::Parser;
use task_tracker::Args;

fn main() {
    let args = Args::parse();

    println!("Hello, {}", args.name);
}
