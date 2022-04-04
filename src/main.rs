mod exec;

use exec::execute;

use clap::Parser;

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

    args.command.iter().for_each(|cmd| execute(cmd).expect(""));
}
