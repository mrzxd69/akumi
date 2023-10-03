use std::{
    fmt::Display,
    fs::{File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

use crate::{enums::OperationsError, Config};

pub fn controller(
    Config {
        lines,
        path,
        numeration,
        delete,
        create,
        append_text,
    }: Config,
) {
    if lines.is_some() {
        match read_file_enumerate(&path, lines.unwrap_or(usize::MAX), numeration) {
            Ok(_) => {}
            Err(err) => println!("{err}"),
        }
    } else if let Some(text) = append_text {
        match append_file_text(path, text) {
            Ok(msg) => println!("{msg}"),
            Err(err) => println!("{err}"),
        }
    } else if delete {
        match delete_file(path) {
            Ok(msg) => println!("{msg}"),
            Err(msg) => println!("{msg}"),
        }
        return;
    } else if create {
        match create_file(path) {
            Ok(msg) => println!("{msg}"),
            Err(msg) => println!("{msg}"),
        }
    } else if append_text.is_none() {
        match read_file(&path, numeration) {
            Ok(_) => {}
            Err(err) => println!("{err}"),
        }
    }
}

fn create_file(path: impl AsRef<Path>) -> Result<String, OperationsError> {
    File::create(path)?.write_all("".as_bytes())?;

    Ok("File successful created ✅".to_string())
}

fn delete_file(path: impl AsRef<Path>) -> Result<String, OperationsError> {
    std::fs::remove_file(path)?;

    Ok("File successful deleted ✨".to_string())
}

fn read_file(path: impl AsRef<Path>, numeration: bool) -> Result<(), OperationsError> {
    let file = File::open(path)?;

    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let success_line = line.map_err(|_| OperationsError::LineRead { linenum: index })?;

        if !numeration {
            println!("{}", success_line)
        } else {
            println!("{} | {}", index + 1, success_line)
        }
    }

    Ok(())
}

fn read_file_enumerate(
    path: impl AsRef<Path>,
    lines_limit: usize,
    enumerate: bool,
) -> Result<(), OperationsError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut line_count = 1;

    for line in reader.lines().take(lines_limit + 1) {
        if line_count < lines_limit + 1 {
            let success_line = line.map_err(|_| OperationsError::LineRead {
                linenum: line_count,
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

fn append_file_text(path: impl AsRef<Path>, text: impl Display) -> Result<String, OperationsError> {
    let file = OpenOptions::new().write(true).append(true).open(path)?;

    let mut file_writer = io::BufWriter::new(file);

    file_writer.write_all(format!("\n{}", text).as_bytes())?;
    file_writer.flush()?;

    Ok("File succesful appened 💫".to_string())
}
