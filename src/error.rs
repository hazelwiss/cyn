use crate::buffers::Cursor;
use crate::tokens::{Delimeter, Punct, TokenTree};
use crate::TokenStream;
use std::fmt::{self, Display};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug)]
pub struct Pos {
    pub col: usize,
    pub row: usize,
}

impl Pos {
    pub fn zero() -> Self {
        Self { col: 0, row: 0 }
    }
}

#[derive(Debug)]
pub struct Error<'a> {
    msg: String,
    file: Option<PathBuf>,
    pos: Option<Pos>,
    cursor: Option<Cursor<'a>>,
}

impl<'a> Error<'a> {
    pub fn new_with_pos(
        msg: impl Display,
        cursor: Option<Cursor<'a>>,
        file: Option<PathBuf>,
        col: usize,
        row: usize,
    ) -> Self {
        Self {
            msg: msg.to_string(),
            file,
            pos: Some(Pos { col, row }),
            cursor,
        }
    }

    pub fn new(msg: impl Display, cursor: Option<Cursor<'a>>) -> Self {
        Self {
            msg: msg.to_string(),
            file: None,
            pos: None,
            cursor,
        }
    }

    pub fn msg(&self) -> &String {
        &self.msg
    }

    pub fn file(&self) -> &Option<PathBuf> {
        &self.file
    }

    pub fn pos(&self) -> Option<Pos> {
        self.pos
    }
}

impl<'a> std::error::Error for Error<'a> {}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            msg,
            file,
            pos,
            cursor,
        } = self;
        let pos = if let Some(Pos { col, row }) = pos {
            let msg = format!("{row}:{col} ");
            if let Some(Some(file)) = file.as_ref().map(|file| file.to_str()) {
                format!("{file} {msg}")
            } else {
                msg
            }
        } else {
            "".to_string()
        };
        f.write_str(&format!(
            "{pos}syntax parsing error '{msg}'{}",
            if let Some(cursor) = cursor {
                let ts = cursor.token_stream();
                let inner = ts.into_inner();
                let mut iter = inner.iter().cloned();
                let mut collected = vec![];
                while let Some(cur) = iter.next() {
                    match &cur.tt {
                        TokenTree::Punct(Punct::SemiColon(_))
                        | TokenTree::Group(Delimeter::Bracket, _) => {
                            collected.push(cur);
                            break;
                        }
                        _ => collected.push(cur),
                    }
                }
                let ts = TokenStream::new(collected.into_boxed_slice());
                format!(" {ts}")
            } else {
                "".to_string()
            }
        ))
    }
}
