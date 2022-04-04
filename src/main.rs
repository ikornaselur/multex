mod exec;

use clap::Parser;
use exec::execute;
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

fn main() {
    let args = Args::parse();

    let mut handles: Vec<_> = Vec::new();

    for command in args.command {
        handles.push(thread::spawn(move || {
            execute(&command).unwrap();
        }));
    }

    for process in handles {
        process.join().unwrap();
    }
}
