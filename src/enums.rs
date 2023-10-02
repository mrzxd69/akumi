

pub enum OperationsError {
    FileReadError(String),
    LineReadError(String),
    FileWriteError(String),
    FileCloseError(String)
}