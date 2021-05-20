use std::{fs, process::Command};

use uuid::Uuid;

use crate::routes::go::Variant;

pub fn compile_go(code: String, variant: Variant) -> Result<String, Box<dyn std::error::Error>> {
    let id = Uuid::new_v4();

    let compiler = match variant {
        Variant::Vanilla => "go",
    };

    fs::write(format!("/tmp/{}.go", id), code.clone())?;

    Command::new(compiler)
        .args(&[
            "build".to_string(),
            "-o".to_string(),
            format!("/tmp/{}", id),
            format!("/tmp/{}.go", id),
        ])
        .output()?;

    Ok(format!("/tmp/{}", id))
}
