extern crate cronenberg;
use cronenberg::cron_item::CronItem;
use cronenberg::cron_item::TimeItem::*;
extern crate notify_rust;
use notify_rust::Notification;

mod cron_writer;
use cron_writer::CronWriter;

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

    let cron_writer = CronWriter { items };
    cron_writer.write();
}
