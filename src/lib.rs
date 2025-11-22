use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arg {
    #[command(subcommand)]
    cmd: Commands,
}

impl Arg {
    pub fn execute(&self) {
        match &self.cmd {
            Commands::Add { task } => println!("Added task :: {task}"),
            Commands::Update { id, task } => println!("Updated task :: {task} with id :: {id}"),
            Commands::Delete { id } => println!("Deleted task with id :: {id}"),
            Commands::Mark(mark_command) => println!("Marked task as :: {:?}", mark_command),
            Commands::List(list_command) => println!("List task :: {:?}", list_command),
        }
    }
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add { task: String },
    Update { id: u32, task: String },
    Delete { id: u32 },
    Mark(MarkCommand),
    List(ListCommand),
}

#[derive(Debug, Args)]
struct MarkCommand {
    id: u32,
    #[command(subcommand)]
    status: MarkStatus,
}

#[derive(Subcommand, Debug, Clone)]
enum MarkStatus {
    InProgress,
    Done,
}

#[derive(Subcommand, Debug, Clone)]
enum ListStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Args)]
struct ListCommand {
    #[command(subcommand)]
    pub status: Option<ListStatus>,
}
