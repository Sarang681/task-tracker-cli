use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arg {
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Add { task: String },
    Update { id: u32, task: String },
    Delete { id: u32 },
    Mark(MarkCommand),
    List(ListCommand),
}

#[derive(Debug, Args)]
pub struct MarkCommand {
    pub id: u32,
    #[command(subcommand)]
    pub status: Option<MarkStatus>,
}

#[derive(Subcommand, Debug, Clone)]
pub enum MarkStatus {
    InProgress,
    Done,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ListStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Args)]
pub struct ListCommand {
    pub id: u32,
    #[command(subcommand)]
    pub status: Option<ListStatus>,
}
