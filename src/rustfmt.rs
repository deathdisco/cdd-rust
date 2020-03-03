use std::process::Command;

pub fn rustfmt(file: &str) -> Result<String, failure::Error> {
    let output = Command::new("rustfmt").args(&[file]).output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    match output.status.success() {
        true => Ok(stdout),
        false => Err(failure::format_err!("{}", stderr)),
    }
}
