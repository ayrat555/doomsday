extern crate cronenberg;
extern crate doomsday;

use doomsday::cron_writer::CronWriter;
use doomsday::cron_writer::CronWriterError;
use doomsday::crontab::Crontab;
use cronenberg::cron_item::CronItem;
use cronenberg::cron_item::TimeItem::*;

#[test]
fn writes_invalid_data_to_crontab() {
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

    let crontab = Crontab { items };
    let cron_writer = CronWriter::default();

    assert_eq!(
        Err(CronWriterError::CrontabError(String::from(
            "\"-\":0: bad day-of-week\ncrontab: errors in crontab file, can\'t install\n"
        ))),
        cron_writer.write(crontab)
    );
}

#[test]
fn writes_valid_data_to_crontab() {
    let items = vec![
        CronItem {
            minute: AllValues,
            hour: AllValues,
            day_of_month: AllValues,
            month: MultipleValues(vec![1, 2, 5]),
            day_of_week: SingleValue(1),
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

    let crontab = Crontab { items };
    let cron_writer = CronWriter::default();

    assert_eq!(Ok(()), cron_writer.write(crontab));
}
