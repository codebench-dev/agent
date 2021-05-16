use std::{
    io,
    process::{Command, Output},
};

pub fn exec_python2(path: String) -> io::Result<Output> {
    Command::new("python2").arg(path).output()
}

pub fn exec_python3(path: String) -> io::Result<Output> {
    Command::new("python3").arg(path).output()
}
