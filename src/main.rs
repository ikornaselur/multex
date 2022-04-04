mod exec;

use anyhow::Result;
use exec::execute;

fn main() -> Result<()> {
    execute("git", vec!["status"])?;
    Ok(())
}
