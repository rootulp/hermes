use eyre::eyre;
use std::process::Stdio;
use std::str;
use std::{io::Write, process::Command};
use tracing::{debug, trace};

use crate::error::{handle_exec_error, handle_generic_error, Error};

pub struct ExecOutput {
    pub stdout: String,
    pub stderr: String,
}

pub fn simple_exec(desc: &str, command_path: &str, args: &[&str]) -> Result<ExecOutput, Error> {
    debug!(
        "Executing command for {}: {} {}",
        desc,
        command_path,
        itertools::join(args, " ")
    );

    let output = Command::new(command_path)
        .args(args)
        .output()
        .map_err(handle_exec_error(command_path))?;

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout)
            .map_err(handle_generic_error)?
            .to_string();

        let stderr = str::from_utf8(&output.stderr)
            .map_err(handle_generic_error)?
            .to_string();

        trace!(
            "command executed successfully with stdout: {}, stderr: {}",
            stdout,
            stderr
        );

        Ok(ExecOutput { stdout, stderr })
    } else {
        let message = str::from_utf8(&output.stderr).map_err(handle_generic_error)?;

        Err(Error::generic(eyre!(
            "command exited with error status {:?} and message: {}",
            output.status.code(),
            message
        )))
    }
}

pub fn exec_with_input(
    desc: &str,
    command_path: &str,
    args: &[&str],
    input: &str,
) -> Result<ExecOutput, Error> {
    debug!(
        "Executing command for {}: {} {}",
        desc,
        command_path,
        itertools::join(args, " ")
    );

    if let Ok(child) = Command::new(command_path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .args(args)
        .spawn()
    {
        let _ = child
            .stdin
            .as_ref()
            .unwrap()
            .write(input.as_bytes());
        let output = child.wait_with_output()?;

        if output.status.success() {
            let stdout = str::from_utf8(&output.stdout)
                .map_err(handle_generic_error)?
                .to_string();

            let stderr = str::from_utf8(&output.stderr)
                .map_err(handle_generic_error)?
                .to_string();

            trace!(
                "command executed successfully with stdout: {}, stderr: {}",
                stdout,
                stderr
            );
            Ok(ExecOutput { stdout, stderr })
        } else {
            let message = str::from_utf8(&output.stderr).map_err(handle_generic_error)?;

            Err(Error::generic(eyre!(
                "command exited with error status {:?} and message: {}",
                output.status.code(),
                message
            )))
        }
    } else {
        Err(Error::generic(eyre!("command with input failed")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_cmd() {
        let desc = "chain";
        let cmd = "gaiad";
        let args = vec![
            "keys",
            "add",
            "cosmos1",
            "--recover",
            "--keyring-backend",
            "test",
        ];
        let input = "fiction perfect rapid steel bundle giant blade grain eagle wing cannon fever must humble dance kitchen lazy episode museum faith off notable rate flavor";
        let o = exec_with_input(desc, cmd, &args, input);
        assert!(o.is_ok(), "error: {:?}", o.err());
    }
}
