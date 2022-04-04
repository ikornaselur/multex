use anyhow::Result;
use clap::Parser;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{Command, Stdio};
use std::thread;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// One or more comamnd to execute in parallel
    ///
    /// A list of one or more command strings that should be executed in the same directory.
    #[clap(required = true)]
    command: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut handles: Vec<_> = Vec::new();

    for command in args.command {
        handles.push(thread::spawn(move || -> Result<()> {
            let stdout = Command::new("sh")
                .arg("-c")
                .arg(command)
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
        }));
    }

    for process in handles {
        process.join().expect("Failed to join thread")?;
    }

    Ok(())
}
