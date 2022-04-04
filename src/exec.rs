use anyhow::Result;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};

pub fn execute(cmd: &str) -> Result<()> {
    let stdout = Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output"))?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .filter_map(|line| line.ok())
        .for_each(|line| println!("{}", line));

    Ok(())
}
