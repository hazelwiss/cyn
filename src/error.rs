use std::fmt::{self, Display};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Error {
    msg: String,
    file: Option<PathBuf>,
    col: usize,
    row: usize,
}

impl Error {
    pub fn new(msg: impl Display, file: Option<PathBuf>, col: usize, row: usize) -> Self {
        Self {
            msg: msg.to_string(),
            file,
            col,
            row,
        }
    }

    pub fn msg(&self) -> &String {
        &self.msg
    }

    pub fn file(&self) -> &Option<PathBuf> {
        &self.file
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn row(&self) -> usize {
        self.row
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            msg,
            file,
            col,
            row,
        } = self;
        let msg = format!("{row}:{col} {msg}");
        f.write_str(&match file {
            Some(file) => {
                if let Some(file) = file.to_str() {
                    format!("{file} {msg}")
                } else {
                    msg
                }
            }
            _ => msg,
        })
    }
}
