use std::{
    ffi::OsStr,
    io,
    path::PathBuf,
    process::{Command, ExitStatus, Output},
    string::FromUtf8Error,
};

use log::debug;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("git command execution failed: {0}")]
    CommandExecFailed(#[from] io::Error),

    #[error("git command failed ({code}): {message}")]
    NonZeroExit { code: i32, message: String },

    #[error("failed transforming output to utf8 string: {0}")]
    Utf8Transform(#[from] FromUtf8Error),
}

pub struct CommandOutput(Output);

#[allow(unused)]
impl CommandOutput {
    pub fn status(&self) -> &ExitStatus {
        &self.0.status
    }

    pub fn stdout(&self) -> Result<String, Error> {
        Ok(String::from_utf8(self.0.stdout.to_vec())?)
    }

    pub fn stderr(&self) -> Result<String, Error> {
        Ok(String::from_utf8(self.0.stderr.to_vec())?)
    }
}

pub struct Git {
    dir: PathBuf,
}

impl Git {
    pub fn new<P>(exec_dir: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            dir: exec_dir.into(),
        }
    }

    pub fn exec<I, S>(&self, args: I) -> Result<CommandOutput, Error>
    where
        I: Clone,
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        debug!(
            "git command: git {}",
            args.clone()
                .into_iter()
                .map(|s| s.as_ref().to_string_lossy().to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );

        let mut cmd = Command::new("git");
        cmd.args(args);
        cmd.current_dir(&self.dir);

        let output = CommandOutput(cmd.output()?);
        if !output.status().success() {
            let message = {
                let stderr = output.stderr()?;
                if !stderr.trim().is_empty() {
                    stderr
                } else {
                    output.stdout()?
                }
            };
            return Err(Error::NonZeroExit {
                code: output.status().code().unwrap_or(-1),
                message,
            });
        }

        Ok(output)
    }

    pub fn current_branch(&self) -> Result<String, Error> {
        let out = self.exec(["branch", "--show-current"])?;
        let branch = out.stdout()?;
        Ok(branch.trim().to_owned())
    }
}
