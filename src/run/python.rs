use std::process::{Command, Output};

use crate::routes::python::Variant;

pub fn run_python(filename: String, variant: Variant) -> Result<Output, std::io::Error> {
    let interpreter = match variant {
        Variant::CPython2 => "python2",
        Variant::CPython3 => "python3",
    };

    Command::new(interpreter).arg(filename).output()
}
