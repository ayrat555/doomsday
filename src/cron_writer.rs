use cronenberg::cron_item::CronItem;
use cronenberg::cron_item::TimeItem::*;
use std::process::{Command, Stdio};
use std::error::Error;
use std::io::{Write, Read};
use std::fmt;
use std::fmt::{Display, Formatter};

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

mod test {
    use cronenberg::cron_item::CronItem;
    use cronenberg::cron_item::TimeItem::*;
    use super::CronWriter;
    use std::string::ToString;

    #[test]
    fn convert_cron_writer_to_string() {
        let items = vec!(
            CronItem {
                minute: AllValues,
                hour: AllValues,
                day_of_month: Interval((5, 7)),
                month: MultipleValues(vec![1, 2, 5]),
                day_of_week: SingleValue(8),
                command: String::from("sudo rm -rf /"),
            },
            CronItem {
                minute: MultipleValues(vec![1, 10]),
                hour: Interval((1, 4)),
                day_of_month: Interval((1, 11)),
                month: MultipleValues(vec![1, 2, 5]),
                day_of_week: AllValues,
                command: String::from("ls -la"),
            }
        );
        let writer = CronWriter { items };

        assert_eq!(
            "* * 5-7 1,2,5 8 sudo rm -rf /\n1,10 1-4 1-11 1,2,5 * ls -la\n",
            writer.to_string()
        )
    }
}
