use anyhow::Result;
use clap::Parser;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
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
    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{prefix:.bold.dim} {spinner} {wide_msg}");

    let mp = MultiProgress::new();
    for command in args.command {
        let pb = mp.add(ProgressBar::new(1));
        pb.set_style(spinner_style.clone());
        pb.set_prefix(format!("{}:", command));

        let _ = thread::spawn(move || -> Result<()> {
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
                .for_each(|line| {
                    pb.set_message(line);
                });
            pb.finish_with_message("Done!");
            Ok(())
        });
    }

    mp.join().unwrap();

    Ok(())
}
