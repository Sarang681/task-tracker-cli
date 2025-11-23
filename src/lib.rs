use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufReader, Error, Read, Write},
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
    if let Ok(id) = create_file_write_all("./tasks.json", task) {
        println!("Task added successfully (ID: {id})");
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

fn create_file_write_all(file_path: &str, contents: &str) -> Result<u32, io::Error> {
    let mut file_contents = String::new();
    match File::open(file_path) {
        Ok(file_handler) => {
            let mut buf_reader = BufReader::new(&file_handler);

            if let Err(e) = buf_reader.read_to_string(&mut file_contents) {
                println!("Unable to read file contents");
                return Err(e);
            }
        }
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                if let Err(e) = fs::File::create(file_path) {
                    println!("Failed to create file at :: {file_path}");
                    return Err(e);
                }
            }
            _ => {
                println!("Failed to create file at :: {file_path}");
                return Err(e);
            }
        },
    }
    let mut tasks: Vec<Task> = if !file_contents.is_empty() {
        match serde_json::from_str(&file_contents) {
            Ok(tasks) => tasks,
            Err(e) => {
                println!("Unable to parse file contents");
                let serde_error = Error::new(io::ErrorKind::InvalidData, e);
                return Err(serde_error);
            }
        }
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

    let new_file_contents;
    match serde_json::to_string(&tasks) {
        Ok(contents) => new_file_contents = contents,
        Err(e) => {
            println!("Unable to serialze new task list");
            let serde_error = Error::new(io::ErrorKind::InvalidData, e);
            return Err(serde_error);
        }
    }
    let mut file;
    match OpenOptions::new()
        .write(true)
        .create(true) // Create the file if it doesn't exist
        .truncate(true) // Truncate the file to zero length, effectively overwriting
        .open(file_path)
    {
        Ok(file_handler) => file = file_handler,
        Err(e) => {
            println!("Unable to open file in write mode");
            return Err(e);
        }
    }
    if let Err(e) = file.write_all(new_file_contents.as_bytes()) {
        println!("Failed to write to file");
        return Err(e);
    }
    Ok(id)
}
