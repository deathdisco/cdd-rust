use std::collections::HashSet;
use std::io::Write;
use std::process::{Command, Stdio};

pub(crate) fn rustfmt(input: &str) -> Result<String, failure::Error> {
    let mut child = Command::new("rustfmt").stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child.stdin
        .as_mut()
        .map(|child| child.write_all(&input.as_bytes())).expect("rustfmt");

    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8(output.stdout)?)
    } else {
        Err(failure::format_err!("rustfmt: {}", String::from_utf8(output.stderr)?))
    }
}