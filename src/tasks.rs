use std::io::{self, Error};

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::{ListCommand, MarkCommand, MarkStatus, TaskStatus, files};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: u32,
    description: String,
    status: TaskStatus,
    created_at: String,
    updated_at: String,
}

pub fn add_task(task: &str) {
    if let Ok(id) = handle_add_task(task) {
        println!("Task added successfully (ID: {id})");
    }
}

pub fn update_task_by_id(id: u32, task: &str) {
    if let Ok(()) = handle_update_task(id, task) {
        println!("Task with id : {} updated succesfully!", id);
    }
}

pub fn delete_task_by_id(id: u32) {
    if let Ok(()) = handle_delete_task(id) {
        println!("Task with id : {} deleted successfully!", id);
    }
}

pub fn mark_task_by_id(mark_command: &MarkCommand) {
    if let Ok(()) = handle_mark_task(mark_command) {
        println!(
            "Successfully marked task with id : {} as {:?}",
            mark_command.id, mark_command.status
        );
    }
}

pub fn list_tasks(list_command: &ListCommand) {
    if let Ok(task) = handle_list_tasks(list_command) {
        if let Ok(stringified_tasks) = files::convert_tasks_to_string(&task) {
            println!("Tasks :: {}", stringified_tasks);
        } else {
            println!("Error fetching tasks");
        }
    }
}

fn handle_add_task(contents: &str) -> Result<u32, io::Error> {
    let file_path = files::fetch_file_path()?;
    let file_contents = files::read_from_file_and_creat_if_not_exists(&file_path)?;
    let mut tasks = files::extract_tasks_from_file_contents(&file_contents)?;
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

    let new_file_contents = files::convert_tasks_to_string(&tasks)?;
    files::write_contents_to_file(&file_path, &new_file_contents)?;
    Ok(id)
}

fn handle_update_task(id: u32, contents: &str) -> Result<(), io::Error> {
    let file_path = files::fetch_file_path()?;
    let file_contents = files::extract_file_contents_from_file(&file_path)?;
    let mut tasks = files::extract_tasks_from_file_contents(&file_contents)?;

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
    let new_file_contents = files::convert_tasks_to_string(&tasks)?;
    files::write_contents_to_file(&file_path, &new_file_contents)?;
    Ok(())
}

fn handle_delete_task(id: u32) -> Result<(), io::Error> {
    let file_path = files::fetch_file_path()?;
    let file_contents = files::extract_file_contents_from_file(&file_path)?;

    let mut tasks = files::extract_tasks_from_file_contents(&file_contents)?;

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
    files::write_contents_to_file(&file_path, &new_file_contents)?;
    Ok(())
}

fn handle_mark_task(mark_command: &MarkCommand) -> Result<(), Error> {
    let file_path = files::fetch_file_path()?;
    let file_contents = files::extract_file_contents_from_file(&file_path)?;

    let mut tasks = files::extract_tasks_from_file_contents(&file_contents)?;

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

    let new_file_contents = files::convert_tasks_to_string(&tasks)?;
    files::write_contents_to_file(&file_path, &new_file_contents)?;
    Ok(())
}

fn handle_list_tasks(list_command: &ListCommand) -> Result<Vec<Task>, Error> {
    let file_path = files::fetch_file_path()?;
    let file_contents = files::extract_file_contents_from_file(&file_path)?;

    let tasks = files::extract_tasks_from_file_contents(&file_contents)?;

    if let Some(filter) = &list_command.status {
        let filtered_tasks = tasks.into_iter().filter(|t| t.status == *filter).collect();
        Ok(filtered_tasks)
    } else {
        Ok(tasks)
    }
}
