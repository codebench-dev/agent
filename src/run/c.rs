use std::{fs, process::Command};

use uuid::Uuid;

use crate::routes::c::Variant;

pub fn compile_c(code: String, variant: Variant) -> Result<String, Box<dyn std::error::Error>> {
    let id = Uuid::new_v4();

    let compiler = match variant {
        Variant::Clang => "clang",
        Variant::GCC => "gcc",
    };

    fs::write(format!("/tmp/{}.c", id), code.clone())?;

    Command::new(compiler)
        .args(&[
            format!("/tmp/{}.c", id),
            "-o".to_string(),
            format!("/tmp/{}.out", id),
        ])
        .output()?;

    Ok(format!("/tmp/{}.out", id))
}
