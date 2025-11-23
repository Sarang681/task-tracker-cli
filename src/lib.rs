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
            Commands::Update { id, task } => update_task_by_id(*id, task),
            Commands::Delete { id } => delete_task_by_id(*id),
            Commands::Mark(mark_command) => mark_task_by_id(mark_command),
            Commands::List(list_command) => println!("List task :: {:?}", list_command),
        }
    }
}

fn add_task(task: &str) {
    if let Ok(id) = handle_add_task("./tasks.json", task) {
        println!("Task added successfully (ID: {id})");
    }
}

fn update_task_by_id(id: u32, task: &str) {
    if let Ok(()) = handle_update_task("./tasks.json", id, task) {
        println!("Task with id : {} updated succesfully!", id);
    }
}

fn delete_task_by_id(id: u32) {
    if let Ok(()) = handle_delete_task("./tasks.json", id) {
        println!("Task with id : {} deleted successfully!", id);
    }
}

fn mark_task_by_id(mark_command: &MarkCommand) {
    if let Ok(()) = handle_mark_task("./tasks.json", mark_command) {
        println!(
            "Successfully marked task with id : {} as {:?}",
            mark_command.id, mark_command.status
        );
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

fn handle_add_task(file_path: &str, contents: &str) -> Result<u32, io::Error> {
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
    let mut tasks = extract_tasks_from_file_contents(file_contents)?;
    let id;
    if let Some(last_task) = tasks.last() {
        id = last_task.id + 1;
    } else {
        id = 1;
    }
    let binding = Local::now().to_string();
    let current_time: &str = binding.as_str();
    let new_task = Task {
        id,
        description: contents.to_string(),
        status: TaskStatus::Todo,
        created_at: current_time.to_string(),
        updated_at: current_time.to_string(),
    };

    tasks.push(new_task);

    let new_file_contents = convert_tasks_to_string(tasks)?;
    write_contents_to_file(file_path, new_file_contents)?;
    Ok(id)
}

fn handle_update_task(file_path: &str, id: u32, contents: &str) -> Result<(), io::Error> {
    let file_contents = extract_file_contents_from_file(file_path)?;
    let mut tasks = extract_tasks_from_file_contents(file_contents)?;

    let task = tasks.iter_mut().find(|t| t.id == id);
    match task {
        Some(t) => {
            t.description = contents.to_string();
            t.updated_at = Local::now().to_string();
        }
        None => {
            println!("Task with id :: {id} not found!");
            let err = Error::new(io::ErrorKind::NotFound, "Task with id :: {id} not found!");
            return Err(err);
        }
    }
    let new_file_contents = convert_tasks_to_string(tasks)?;
    write_contents_to_file(file_path, new_file_contents)?;
    Ok(())
}

fn handle_delete_task(file_path: &str, id: u32) -> Result<(), io::Error> {
    let file_contents = extract_file_contents_from_file(file_path)?;

    let mut tasks = extract_tasks_from_file_contents(file_contents)?;

    if let None = tasks.iter().find(|task| task.id == id) {
        println!("Task with id : {} not found!", id);
        let not_found_error = Error::new(io::ErrorKind::NotFound, "Task with id : {} not found!");
        return Err(not_found_error);
    }

    tasks.retain(|task| task.id != id);

    let new_file_contents = match serde_json::to_string(&tasks) {
        Ok(value) => value,
        Err(e) => {
            println!("Error writing tasks to file");
            let serde_error = Error::new(io::ErrorKind::InvalidData, e);
            return Err(serde_error);
        }
    };
    write_contents_to_file(file_path, new_file_contents)?;
    Ok(())
}

fn handle_mark_task(file_path: &str, mark_command: &MarkCommand) -> Result<(), Error> {
    let file_contents = extract_file_contents_from_file(file_path)?;

    let mut tasks = extract_tasks_from_file_contents(file_contents)?;

    if let Some(task) = tasks.iter_mut().find(|task| task.id == mark_command.id) {
        let task_status = match mark_command.status {
            MarkStatus::InProgress => TaskStatus::InProgress,
            MarkStatus::Done => TaskStatus::Done,
        };
        task.status = task_status;
        task.updated_at = Local::now().to_string();
    } else {
        println!("Task with id : {} not found!", mark_command.id);
        let not_found_error = Error::new(io::ErrorKind::NotFound, "Task with id : {} not found!");
        return Err(not_found_error);
    }

    let new_file_contents = convert_tasks_to_string(tasks)?;
    write_contents_to_file(file_path, new_file_contents)?;
    Ok(())
}

fn extract_file_contents_from_file(file_path: &str) -> Result<String, Error> {
    let mut file_contents = String::new();
    match File::open(file_path) {
        Ok(file_handler) => {
            let mut buf_reader = BufReader::new(&file_handler);

            if let Err(e) = buf_reader.read_to_string(&mut file_contents) {
                println!("Unable to read file contents");
                return Err(e);
            }
        }
        Err(e) => {
            println!("Failed to read file at :: {file_path}");
            return Err(e);
        }
    }
    Ok(file_contents)
}

fn extract_tasks_from_file_contents(file_contents: String) -> Result<Vec<Task>, Error> {
    let tasks: Vec<Task> = if !file_contents.is_empty() {
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
    Ok(tasks)
}

fn write_contents_to_file(file_path: &str, new_file_contents: String) -> Result<(), Error> {
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
    Ok(())
}

fn convert_tasks_to_string(tasks: Vec<Task>) -> Result<String, Error> {
    let new_file_contents;
    match serde_json::to_string(&tasks) {
        Ok(contents) => new_file_contents = contents,
        Err(e) => {
            println!("Unable to serialze new task list");
            let serde_error = Error::new(io::ErrorKind::InvalidData, e);
            return Err(serde_error);
        }
    }
    Ok(new_file_contents)
}
