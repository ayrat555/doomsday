use cronenberg::cron_item::CronItem;
use cronenberg::cron_item::TimeItem::*;
use std::process::{Child, Command, Stdio};
use std::error::Error;
use std::io::{Read, Write};
use std::fmt;
use std::fmt::{Display, Formatter};
use self::CronWriterError::*;

#[derive(Debug)]
pub struct CronWriter {
    pub items: Vec<CronItem>,
}

impl Display for CronWriter {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut acc = String::from("");

        for item in &self.items {
            acc.push_str(item.to_string().as_str());
            acc.push_str("\n");
        }

        write!(f, "{}", acc)
    }
}

pub enum CronWriterError {
    ProcessSpawnError(String),
    CrontabStdinError(String),
    CrontabError(String),
}

impl CronWriter {
    pub fn write(&self) -> Result<(), CronWriterError> {
        let process = start_crontab_process()?;

        write_data(self, &process)?;

        read_process_output(process)
    }
}

fn start_crontab_process() -> Result<Child, CronWriterError> {
    match Command::new("crontab")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
    {
        Err(er) => Err(ProcessSpawnError(String::from(er.description()))),
        Ok(process) => Ok(process),
    }
}

fn write_data(cron_writer: &CronWriter, process: &Child) -> Result<(), CronWriterError> {
    match process
        .stdin
        .unwrap()
        .write_all(cron_writer.to_string().as_bytes())
    {
        Err(ref message) => Err(CrontabStdinError(String::from(message.description()))),
        Ok(_) => Ok(println!("wrote data to crontab")),
    }
}

fn read_process_output(process: Child) -> Result<(), CronWriterError> {
    let mut s = String::new();

    match process.stdout.unwrap().read_to_string(&mut s) {
        Err(message) => Err(CrontabError(String::from(message.description()))),
        Ok(_) => Ok(print!("crontab responded with:\n{}", s)),
    }
}

mod test {
    use cronenberg::cron_item::CronItem;
    use cronenberg::cron_item::TimeItem::*;
    use super::CronWriter;
    use std::string::ToString;

    #[test]
    fn convert_cron_writer_to_string() {
        let items = vec![
            CronItem {
                minute: AllValues,
                hour: AllValues,
                day_of_month: Interval((5, 7)),
                month: MultipleValues(vec![1, 2, 5]),
                day_of_week: SingleValue(8),
                command: String::from("pwd"),
            },
            CronItem {
                minute: MultipleValues(vec![1, 10]),
                hour: Interval((1, 4)),
                day_of_month: Interval((1, 11)),
                month: MultipleValues(vec![1, 2, 5]),
                day_of_week: AllValues,
                command: String::from("ls -la"),
            },
        ];
        let writer = CronWriter { items };

        assert_eq!(
            "* * 5-7 1,2,5 8 pwd\n1,10 1-4 1-11 1,2,5 * ls -la\n",
            writer.to_string()
        )
    }
}
