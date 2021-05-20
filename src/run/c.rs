use std::{fs, process::Command};

use uuid::Uuid;

use anyhow::{anyhow, Result};

use crate::routes::c::Variant;

pub fn compile_c(code: String, variant: Variant) -> Result<String> {
    let id = Uuid::new_v4();

    let compiler = match variant {
        Variant::Clang => "clang",
        Variant::GCC => "gcc",
    };

    fs::write(format!("/tmp/{}.c", id), code.clone())?;

    let output = Command::new(compiler)
        .args(&[
            format!("/tmp/{}.c", id),
            "-o".to_string(),
            format!("/tmp/{}.out", id),
        ])
        .output()?;

    match output.status.success() {
        true => Ok(format!("/tmp/{}.out", id)),
        false => Err(anyhow!(String::from_utf8_lossy(&output.stderr).to_string())),
    }
}
