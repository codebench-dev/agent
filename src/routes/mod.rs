use serde::{Deserialize, Serialize};

pub mod c;
pub mod cmd;
pub mod health;
pub mod python;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunRes {
    message: String,
    stdout: String,
    stderr: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RunCodeReq {
    id: String,
    code: String,
    variant: String,
}
