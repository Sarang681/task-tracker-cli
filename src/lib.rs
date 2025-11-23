use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

mod config;
mod files;
mod tasks;

#[derive(Parser, Debug)]
#[command(
    author = "Sarang Desai",
    version = "0.1.0",
    about = "A fast, minimal CLI tool for adding, viewing, and completing tasks.",
    long_about = "A lightweight terminal-based task manager that helps you quickly track, organize, and complete your to-dos with simple commands."
)]
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
    /// Add a new task to your list.
    Add {
        /// The description of the task to add.
        task: String,
    },
    /// Update the description of an existing task by ID.
    Update {
        /// The ID of the task to update.
        id: u32,
        /// The new description for the task.
        task: String,
    },
    /// Remove a task from your list by ID.
    Delete {
        /// The ID of the task to delete.
        id: u32,
    },
    /// Change the status of a task
    Mark(
        /// Options specifying which task to mark and how to mark it.
        MarkCommand,
    ),
    /// View tasks, optionally filtered.
    List(
        /// Options that control how tasks are listed.
        ListCommand,
    ),
}

/// Defines the options for marking a task with a new status.
#[derive(Debug, Args)]
struct MarkCommand {
    /// The ID of the task to update.
    id: u32,
    /// The status to apply to the task (e.g., `in-progress`, `done`).
    #[command(subcommand)]
    status: MarkStatus,
}

/// Represents the status a task can be updated to when using the `mark` command.
#[derive(Subcommand, Debug, Clone)]
enum MarkStatus {
    /// Mark the task as currently in progress.
    InProgress,
    /// Mark the task as completed.
    Done,
}

/// Represents the current status of a task in the task tracker.
#[derive(Subcommand, Debug, Clone, Serialize, Deserialize, PartialEq)]
enum TaskStatus {
    /// The task has not been started yet.
    Todo,
    /// The task is currently being worked on.
    InProgress,
    /// The task has been completed.
    Done,
}

/// Defines options for listing tasks in the task tracker.
#[derive(Debug, Args)]
struct ListCommand {
    /// Optionally filter the listed tasks by their status
    /// (e.g., `todo`, `in-progress`, `done`).
    #[command(subcommand)]
    pub status: Option<TaskStatus>,
}
