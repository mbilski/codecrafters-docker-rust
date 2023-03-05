use std::process::Stdio;

use anyhow::{Context, Result};

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();

    let command = &args[3];
    let command_args = &args[4..];

    let process = std::process::Command::new(command)
        .args(command_args)
        .stdin(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .and_then(|mut s| s.wait())
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                command, command_args
            )
        })?;

    if !process.success() {
        std::process::exit(1);
    }

    Ok(())
}
