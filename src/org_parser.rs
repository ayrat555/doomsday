use chrono::prelude::*;
use chrono::ParseError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum OrgParseError {
    Message(String)
}

#[derive(Debug, PartialEq)]
pub struct Todo {
    date: String, //DateTime<Local>,
    text: String
}

impl From<ParseError> for OrgParseError {
	fn from(_value: ParseError) -> Self {
		OrgParseError::Message("Could not parse date".into())
	}
}

impl FromStr for Todo {
    type Err = OrgParseError;

    fn from_str(s: &str) -> Result<Self, OrgParseError> {
        if !s.contains("TODO") {
            return Err(OrgParseError::Message("String doesn't contain TODO word".into()));
        }

        if !s.contains("SCHEDULED:") {
            return Err(OrgParseError::Message("TODO Item is not scheduled".into()));
        }

        let mut title: Option<String> = None;
        let mut schedule_date: Option<String> = None;

        for line in s.lines() {
            if line.contains("TODO") {
                title = Some(String::from(line.split("TODO").last().unwrap().trim()));
            }

            if line.contains("SCHEDULED:") {
                let date_string = line.split("SCHEDULED:").last().unwrap().trim();
                schedule_date = Some(date_string.into())
                // schedule_date = Some(date_string.parse::<DateTime<Local>>()?);
            }
        }

        Ok(Todo { date: schedule_date.unwrap(), text: title.unwrap() })
    }
}

#[cfg(test)]
mod tests {
    use super::Todo;
    use std::str::FromStr;
    use super::OrgParseError;
    use chrono::prelude::*;

    #[test]
    fn test_valid_todo_item() {
        let todo_item = "* TODO Gym \n SCHEDULED: <2018-02-11 Sun 11:00>";

        assert_eq!(Todo::from_str(todo_item), Ok(Todo { date: "<2018-02-11 Sun 11:00>".into(), text: "Gym".into() }));
    }

    #[test]
    fn test_invalid_todo_item() {
        let todo_item = "Wrong item";

        assert_eq!(Todo::from_str(todo_item), Err(OrgParseError::Message("String doesn't contain TODO word".into())));
    }
}
