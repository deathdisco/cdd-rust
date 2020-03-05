// use std::process::Command;

// pub(crate) fn rustfmt(file: &str) -> Result<String, failure::Error> {
//     // let output = Command::new("rustfmt").args(&[file]).output()?;
//     let cmd = Command::new("rustfmt")
//         .stdin(Stdio::piped())
//         .output()?;
//     let stdout = String::from_utf8(output.stdout)?;
//     let stderr = String::from_utf8(output.stderr)?;

//     match output.status.success() {
//         true => Ok(stdout),
//         false => Err(failure::format_err!("rustfmt: {}", stderr)),
//     }
// }

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
        // .or(Err(failure::format_err!("Child process stdin has not been captured!")))?;
        // .write_all(file)?;

    let output = child.wait_with_output()?;

    if output.status.success() {
        let raw_output = String::from_utf8(output.stdout)?;
        // let words = raw_output.split_whitespace()
        //     .map(|s| s.to_lowercase())
        //     .collect::<HashSet<_>>();
        // println!("Found {} unique words:", words.len());
        // println!("{:#?}", words);
        Ok(raw_output)
    } else {
        // let err = String::from_utf8(output.stderr)?;
        Err(failure::format_err!("rustfmt: {}", String::from_utf8(output.stderr)?))
    }
    // Ok("".to_string())
}