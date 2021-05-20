use std::{
    io,
    process::{Command, Output},
};

pub fn exec_binary(path: String) -> io::Result<Output> {
    Command::new(path).output()
}
