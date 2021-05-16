use std::{fs, process::Command};

use uuid::Uuid;

pub fn compile_cpp(code: String) -> Result<String, Box<dyn std::error::Error>> {
    let id = Uuid::new_v4();

    fs::write(format!("/tmp/{}.cpp", id), code.clone())?;

    Command::new("g++")
        .args(&[
            format!("/tmp/{}.cpp", id),
            "-o".to_string(),
            format!("/tmp/{}.out", id),
        ])
        .output()?;

    Ok(format!("/tmp/{}.out", id))
}
