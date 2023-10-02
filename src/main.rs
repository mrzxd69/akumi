use std::env;
use regex::Regex;

mod enums;
mod operations;

#[derive(Default, Debug)]
pub struct Config {
    lines: Option<usize>,
    path: String,
    numeration: bool,
    delete: bool,
    create: bool,
    append_text: Option<String>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::default();
    
    parse_args(args, config);
}


fn parse_args(args: Vec<String>, mut config: Config) {
    let regular = Regex::new(r"[^/\\]+\.[a-zA-Z0-9]+$").unwrap();

    for (index, arg) in args.iter().enumerate() {
        
        if regular.is_match(arg) {
            config.path = arg.to_string();
        }
    
        match arg.as_str() {
            "--file" | "-f" => {
                if let Ok(file) = args[index + 1].parse::<String>() {
                    config.path = file;
                }

                if regular.is_match(arg) {
                    config.path = arg.to_string();
                }
            },
            "--lines" | "-l" => {
                if let Ok(lines) = args[index + 1].parse::<usize>() {
                    config.lines = Some(lines);
                } else {
                    config.lines = None;
                }
            },    
            "--append" | "-a" => {
                if let Ok(append_file) = args[index + 1].parse::<String>() {
                    config.append_text = Some(append_file);
                }
            },
            "--numeration" | "-n" => config.numeration = true,
            "--delete" | "-d" => config.delete = true,
            "--create" | "-c" => config.create = true,
            &_ => {}
        };
    }

    if config.path.is_empty() {
        eprintln!("Пожалуйста, укажите файл");
        std::process::exit(1);
    }
    
    operations::controller(config);
}