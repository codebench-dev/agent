use std::{fs, process::Command};

use uuid::Uuid;

pub fn compile_c(code: String) -> Result<String, Box<dyn std::error::Error>> {
    let id = Uuid::new_v4();

    fs::write(format!("/tmp/{}.c", id), code.clone())?;

    Command::new("gcc")
        .args(&[
            format!("/tmp/{}.c", id),
            "-o".to_string(),
            format!("/tmp/{}.out", id),
        ])
        .output()?;

    Ok(format!("/tmp/{}.out", id))
}
