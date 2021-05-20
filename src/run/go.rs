use std::{fs, process::Command};

use uuid::Uuid;

use anyhow::{anyhow, Result};

use crate::routes::go::Variant;

pub fn compile_go(code: String, variant: Variant) -> Result<String> {
    let id = Uuid::new_v4();

    let compiler = match variant {
        Variant::Vanilla => "go",
    };

    fs::write(format!("/tmp/{}.go", id), code.clone())?;

    let output = Command::new(compiler)
        .args(&[
            "build".to_string(),
            "-o".to_string(),
            format!("/tmp/{}", id),
            format!("/tmp/{}.go", id),
        ])
        .output()?;

    match output.status.success() {
        true => Ok(format!("/tmp/{}", id)),
        false => Err(anyhow!(String::from_utf8_lossy(&output.stderr).to_string())),
    }
}
