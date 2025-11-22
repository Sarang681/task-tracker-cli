use clap::Parser;
use task_tracker::Arg;

fn main() {
    let args = Arg::parse();

    args.execute();
}
