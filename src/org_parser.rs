use chrono::prelude::*;
use std::str::FromStr;

struct OrgParseError {
    desc: String
}

pub struct Todo {
    date: DateTime<Local>,
    text: String
}

impl FromStr for Todo {
    type Err = OrgParseError;

    fn from_str(s: &str) -> Result<Self, OrgParseError> {
        if !s.contains("TODO") {
            return Err(OrgParseError { desc: String::from("String doesn't contain TODO word") });
        }

        if !s.contains("SCHEDULED:") {
            return Err(OrgParseError { desc: String::from("TODO Item is not scheduled") });
        }

        let mut title;
        let mut schedule_date;

        for line in s.lines() {
            if line.contains("TODO") {
                title = String::from(line.split("TODO").last().unwrap());
            }

            if line.contains("SCHEDULED:") {
                let date_string = line.split("SCHEDULED:").last().unwrap();
                let schedule_date = date_string.parse::<DateTime<Local>>().unwrap();
            }
        }

        Ok(Todo { date: schedule_date, text: title })
    }
}
