use anyhow::Result;

use std::process::Command;

pub fn execute(cmd: &str, args: Vec<&str>) -> Result<()> {
    let output = Command::new(cmd).args(args).output()?;

    if !output.status.success() {
        anyhow::bail!("Failed to execute: {}", String::from_utf8(output.stderr)?);
    }

    println!("{:?}", String::from_utf8(output.stdout)?);

    Ok(())
}
