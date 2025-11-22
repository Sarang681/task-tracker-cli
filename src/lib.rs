use std::{
    fs::{self, File, OpenOptions},
    io::{BufReader, Read, Write},
};

use chrono::Local;
use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Arg {
    #[command(subcommand)]
    cmd: Commands,
}

impl Arg {
    pub fn execute(&self) {
        match &self.cmd {
            Commands::Add { task } => add_task(task),
            Commands::Update { id, task } => println!("Updated task :: {task} with id :: {id}"),
            Commands::Delete { id } => println!("Deleted task with id :: {id}"),
            Commands::Mark(mark_command) => println!("Marked task as :: {:?}", mark_command),
            Commands::List(list_command) => println!("List task :: {:?}", list_command),
        }
    }
}

fn add_task(task: &str) {
    let id = create_file_write_all("./tasks.json", task);
    println!("Task added successfully (ID: {id})");
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

#[derive(Subcommand, Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: u32,
    description: String,
    status: TaskStatus,
    created_at: String,
    updated_at: String,
}

fn create_file_write_all(file_path: &str, contents: &str) -> u32 {
    if fs::metadata(file_path).is_ok() {
        let file = File::open(file_path).expect("Unable to open file!");
        let mut buf_reader = BufReader::new(&file);
        let mut file_contents = String::new();
        buf_reader
            .read_to_string(&mut file_contents)
            .expect("Unable to read file contents");
        let mut tasks: Vec<Task> = if !file_contents.is_empty() {
            serde_json::from_str(&file_contents).expect("Unable to parse file contents")
        } else {
            Vec::new()
        };
        let id;
        if let Some(last_task) = tasks.last() {
            id = last_task.id + 1;
        } else {
            id = 1;
        }
        let new_task = Task {
            id,
            description: contents.to_string(),
            status: TaskStatus::Todo,
            created_at: Local::now().to_string(),
            updated_at: Local::now().to_string(),
        };

        tasks.push(new_task);
        let new_file_contents =
            serde_json::to_string(&tasks).expect("Unable to serialze new task list");
        let mut file = OpenOptions::new()
            .write(true)
            .create(true) // Create the file if it doesn't exist
            .truncate(true) // Truncate the file to zero length, effectively overwriting
            .open(file_path)
            .expect("Unable to open file in write mode");
        file.write_all(new_file_contents.as_bytes())
            .expect("Failed to write to file");
        id
    } else {
        let mut file =
            fs::File::create(file_path).expect("Failed to create file at :: {file_path}");
        let new_task = Task {
            id: 1,
            description: contents.to_string(),
            status: TaskStatus::Todo,
            created_at: Local::now().to_string(),
            updated_at: Local::now().to_string(),
        };
        let tasks: Vec<Task> = vec![new_task];
        let new_file_contents =
            serde_json::to_string(&tasks).expect("Unable to serialze new task list");
        file.write_all(new_file_contents.as_bytes())
            .expect("Failed to write to file");
        1
    }
}
