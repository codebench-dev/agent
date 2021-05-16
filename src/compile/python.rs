use std::fs;

use uuid::Uuid;

pub fn compile_py(code: String) -> Result<String, Box<dyn std::error::Error>> {
    let id = Uuid::new_v4();

    fs::write(format!("/tmp/{}.py", id), code.clone())?;

    Ok(format!("/tmp/{}.py", id))
}
