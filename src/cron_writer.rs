use cronenberg::cron_item::CronItem;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::error::Error;
use std::io::{Read, Write};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::default::Default;
use self::CronWriterError::*;
use crontab::Crontab;

#[derive(Debug)]
pub struct CronWriter {
    pub cron_command: String,
    pub user: String,
}

pub enum CronWriterError {
    ProcessSpawnError(String),
    CrontabStdinError(String),
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

        read_process_output(&mut process.stdout.unwrap())
    }
}

fn start_crontab_process(command: &str) -> Result<Child, CronWriterError> {
    match Command::new(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(er) => Err(ProcessSpawnError(String::from(er.description()))),
        Ok(process) => Ok(process),
    }
}

fn write_data(string: String, stdin: &mut ChildStdin) -> Result<(), CronWriterError> {
    match stdin.write_all(string.as_bytes()) {
        Err(err) => Err(CrontabStdinError(String::from(err.description()))),
        Ok(_) => Ok(println!("wrote data to crontab")),
    }
}

fn read_process_output(stdout: &mut ChildStdout) -> Result<(), CronWriterError> {
    let mut s = String::new();

    match stdout.read_to_string(&mut s) {
        Err(err) => Err(CrontabError(String::from(err.description()))),
        Ok(_) => Ok(print!("crontab responded with:\n{}", s)),
    }
}
