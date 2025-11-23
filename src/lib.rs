use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

mod files;
mod tasks;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arg {
    #[command(subcommand)]
    cmd: Commands,
}

impl Arg {
    pub fn execute(&self) {
        match &self.cmd {
            Commands::Add { task } => tasks::add_task(task),
            Commands::Update { id, task } => tasks::update_task_by_id(*id, task),
            Commands::Delete { id } => tasks::delete_task_by_id(*id),
            Commands::Mark(mark_command) => tasks::mark_task_by_id(mark_command),
            Commands::List(list_command) => tasks::list_tasks(list_command),
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

#[derive(Subcommand, Debug, Clone, Serialize, Deserialize, PartialEq)]
enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Args)]
struct ListCommand {
    #[command(subcommand)]
    pub status: Option<TaskStatus>,
}
