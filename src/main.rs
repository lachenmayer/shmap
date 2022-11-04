use core::panic;
use std::{
    env, io,
    process::{Command, ExitStatus},
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if let [_, var_name, command] = &args[..] {
        let stdin = io::stdin();
        for maybe_line in stdin.lines() {
            let line = maybe_line?;
            let exit_status = spawn_with_env(command, var_name, &line)?;
            if !exit_status.success() {
                eprintln!(
                    "shmap: command {:?}, ${}={:?} failed with {}",
                    command, var_name, line, exit_status
                );
            }
        }
    } else {
        panic!("Usage: shmap var_name 'command containing $var_name'");
    }
    Ok(())
}

fn spawn_with_env(
    command: &String,
    var_name: &String,
    value: &String,
) -> Result<ExitStatus, std::io::Error> {
    Command::new("/bin/sh")
        .arg("-c")
        .arg(command)
        .env(var_name, value)
        .status()
}
