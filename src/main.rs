extern crate cronenberg;
use cronenberg::cron_item::CronItem;
use cronenberg::cron_item::TimeItem::*;
extern crate notify_rust;
use notify_rust::Notification;

mod cron_writer;
mod crontab;
use cron_writer::CronWriter;
use crontab::Crontab;

fn main() {
    Notification::new()
        .summary("Doomsady")
        .body("The day is coming")
        .icon("doom")
        .show()
        .unwrap();

    let items = vec![
        CronItem {
            minute: AllValues,
            hour: AllValues,
            day_of_month: AllValues,
            month: AllValues,
            day_of_week: AllValues,
            command: String::from("ls -la"),
        },
        CronItem {
            minute: AllValues,
            hour: AllValues,
            day_of_month: AllValues,
            month: AllValues,
            day_of_week: AllValues,
            command: String::from("ls -la"),
        },
    ];
    let crontab = Crontab { items };
    let cron_writer = CronWriter::default();
    cron_writer.write(crontab);
}
