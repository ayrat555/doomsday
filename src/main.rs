extern crate clap;
extern crate cronenberg;
extern crate yaml_rust;
use cronenberg::cron_item::CronItem;
use cronenberg::cron_item::TimeItem::*;
extern crate notify_rust;
use notify_rust::Notification;

mod cron_writer;
mod crontab;
mod notifier;
mod yaml_parser;
use cron_writer::CronWriter;
use crontab::Crontab;
use clap::{App, Arg};
use yaml_parser::parse_notifications_from_yaml;

fn main() {
    let matches = App::new("doomsday")
        .version("0.1.0")
        .about("Notifies about recurring events")
        .author("Ayrat Badykov")
        .arg(
            Arg::with_name("message")
                .short("m")
                .long("message")
                .help("Shows message as a system notification")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("schedule")
                .short("s")
                .long("schedule")
                .help("Schedules system notifications")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("cron command")
                .short("c")
                .long("command")
                .help("Cron command")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("cron user")
                .short("u")
                .long("user")
                .help("Cron user")
                .takes_value(true),
        )
        .get_matches();

    match matches.value_of("message") {
        Some(message) => notifier::notify(message),
        None => (),
    }

    match matches.value_of("schedule") {
        Some(schedule_file) => {
            let crontab = parse_notifications_from_yaml(schedule_file);
            CronWriter::default().write(crontab)?;
        }
        None => (),
    }
}
