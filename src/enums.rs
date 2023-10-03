use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum OperationsError {
    #[error("io occurred")]
    Io(#[from] io::Error),
    #[error("error on line: {linenum}")]
    LineRead { linenum: usize },
}
