use std::process::{Child, ChildStderr, ChildStdin, Command, Stdio};
use std::error::Error;
use std::io::{Read, Write};
use std::default::Default;
use crontab::Crontab;
use self::CronWriterError::*;

#[derive(Debug)]
pub struct CronWriter {
    pub cron_command: String,
    pub user: String,
}

#[derive(Debug, PartialEq)]
pub enum CronWriterError {
    ProcessSpawnError(String),
    CrontabStdinError(String),
    CrontabStderrError(String),
    CrontabError(String),
}

impl CronWriter {
    pub fn command(&self) -> String {
        if self.user == "" {
            return self.cron_command.to_owned();
        }

        format!("{} -u {}", self.cron_command, self.user)
    }
}

impl Default for CronWriter {
    fn default() -> CronWriter {
        CronWriter {
            cron_command: String::from("crontab"),
            user: String::from(""),
        }
    }
}

impl CronWriter {
    pub fn write(&self, crontab: Crontab) -> Result<(), CronWriterError> {
        let process = start_crontab_process(&self.command())?;

        write_data(crontab.to_string(), &mut process.stdin.unwrap())?;
        read_process_errors(&mut process.stderr.unwrap())
    }
}

fn start_crontab_process(command: &str) -> Result<Child, CronWriterError> {
    match Command::new(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Err(er) => Err(ProcessSpawnError(String::from(er.description()))),
        Ok(process) => Ok(process),
    }
}

fn write_data(string: String, stdin: &mut ChildStdin) -> Result<(), CronWriterError> {
    match stdin.write_all(string.as_bytes()) {
        Err(err) => Err(CrontabStdinError(String::from(err.description()))),
        Ok(_) => Ok(()),
    }
}

fn read_process_errors(stdout: &mut ChildStderr) -> Result<(), CronWriterError> {
    let mut s = String::new();

    match stdout.read_to_string(&mut s) {
        Err(err) => Err(CrontabStderrError(String::from(err.description()))),
        Ok(_) => {
            if s == String::from("") {
                Ok(())
            } else {
                Err(CrontabError(String::from(s)))
            }
        }
    }
}

mod test {
    use super::CronWriter;

    #[test]
    fn return_default_command() {
        let cron_writer = CronWriter::default();

        assert_eq!("crontab", cron_writer.command());
    }

    #[test]
    fn build_custom_cron_command() {
        let cron_writer = CronWriter {
            cron_command: String::from("hey_cron"),
            user: String::from("doom"),
        };

        assert_eq!("hey_cron -u doom", cron_writer.command());
    }
}
