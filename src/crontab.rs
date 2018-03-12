use std::fmt::{Display, Formatter};
use std::fmt;
use cronenberg::CronItem;

#[derive(Debug, PartialEq)]
pub struct Crontab {
    pub items: Vec<CronItem>,
}

impl Display for Crontab {
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
    use super::Crontab;
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
        let writer = Crontab { items };

        assert_eq!(
            "* * 5-7 1,2,5 8 pwd\n1,10 1-4 1-11 1,2,5 * ls -la\n",
            writer.to_string()
        )
    }
}
