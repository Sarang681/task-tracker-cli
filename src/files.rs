use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufReader, Error, ErrorKind, Read, Write},
};

use crate::tasks::Task;

pub fn read_from_file_and_creat_if_not_exists(file_path: &str) -> Result<String, Error> {
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
    Ok(file_contents)
}

pub fn extract_file_contents_from_file(file_path: &str) -> Result<String, Error> {
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

pub fn extract_tasks_from_file_contents(file_contents: &str) -> Result<Vec<Task>, Error> {
    let tasks: Vec<Task> = if !file_contents.is_empty() {
        match serde_json::from_str(&file_contents) {
            Ok(tasks) => tasks,
            Err(e) => {
                println!("Unable to parse file contents");
                let serde_error = Error::new(ErrorKind::InvalidData, e);
                return Err(serde_error);
            }
        }
    } else {
        Vec::new()
    };
    Ok(tasks)
}

pub fn write_contents_to_file(file_path: &str, new_file_contents: &str) -> Result<(), Error> {
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

pub fn convert_tasks_to_string(tasks: &Vec<Task>) -> Result<String, Error> {
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
