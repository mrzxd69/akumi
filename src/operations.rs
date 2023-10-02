use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufRead, Write};
use crate::enums::OperationsError;

use crate::Config;

pub fn controller(config: Config) {
    let Config { 
        lines, 
        path,
        numeration, 
        append_text,
        delete,
        create
    } = config;

    if lines.is_some() {
        match read_file_enumerate(&path, lines.unwrap_or(usize::MAX), numeration) {
            Ok(_) => {},
            Err(OperationsError::FileReadError(msg)) => println!("{msg}"),
            Err(OperationsError::LineReadError(msg)) => println!("{msg}"),
            _ => {}
        }
        return;
    }
    if let Some(text) = append_text {
        match append_file_text(path, text) {
            Ok(msg) => println!("{msg}"),
            Err(OperationsError::FileCloseError(msg)) => println!("{msg}"),
            Err(OperationsError::FileReadError(msg)) => println!("{msg}"),
            Err(OperationsError::FileWriteError(msg)) => println!("{msg}"),
            _ => {} 
        }
        return;
    }

    if delete {
        match delete_file(path) {
            Ok(msg) => println!("{msg}"),
            Err(msg) => println!("{msg}")
        }
        return;
    }

    if create {
        match create_file(path) {
            Ok(msg) => println!("{msg}"),
            Err(msg) => println!("{msg}")
        }
        return;
    }

    if append_text.is_none() {
        match read_file(&path, numeration) {
            Ok(_) => {},
            Err(OperationsError::FileReadError(msg)) => println!("{msg}"),
            Err(OperationsError::LineReadError(msg)) => println!("{msg}"),
            _ => {}
        }
        return;
    }
}

fn create_file(path: String) -> Result<String, std::io::Error> {
    File::create(path)?
        .write_all("".as_bytes())?;

    Ok("File successful created ✅".to_string())
}

fn delete_file(path: String) -> Result<String, String> {
    
    if let Err(_) = std::fs::remove_file(&path) {
        return Err("File not found".to_string());
    }

    Ok("File successful deleted ✨".to_string())
}

fn read_file(path: &String, numeration: bool) -> Result<(), OperationsError> {

    let file = File::open(path).map_err(|_| {
        return OperationsError::FileReadError("File not found :(".to_string());
    })?;
    
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().into_iter().enumerate() {
        let success_line = line.map_err(|_| {
            return OperationsError::LineReadError(format!("Error reading on line: {}", index))
        })?;

        if !numeration { 
            println!("{}", success_line) 
        } else { 
            println!("{} | {}", index + 1, success_line) 
        }
    }

   Ok(())
}

fn read_file_enumerate(path: &String, lines_limit: usize, enumerate: bool) -> Result<(), OperationsError> {

    let file = File::open(path).map_err(|_| {
        return OperationsError::FileReadError("File not found :(".to_string());
    })?;
    let reader = BufReader::new(file);
    let mut line_count = 1;

    for line in reader.lines().take(lines_limit + 1) {
        if line_count < lines_limit + 1 { 
            let success_line = line.map_err(|_| {
                return OperationsError::LineReadError(format!("Error reading on line: {}", line_count))
            })?;
    
            if enumerate { 
                println!("{} | {}", line_count, &success_line);
            } else {
                println!("{}", &success_line);
            }
            line_count += 1;
        }
    }
    Ok(())
}


fn append_file_text(path: String, text: String) -> Result<String, OperationsError> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .map_err(|_| {
            return OperationsError::FileReadError("file opening error".to_string());
        })?;

    let mut file_writer = std::io::BufWriter::new(file);

    file_writer.write_all(format!("\n{}", text).as_bytes())
        .map_err(|_| {
            return OperationsError::FileWriteError("File writing error".to_string());
        })?;
    
    file_writer
        .flush()
        .map_err(|_| {
            return OperationsError::FileCloseError("File closing error".to_string());
        })?;

    Ok("File succesful appened 💫".to_string())
}