extern crate clap;
extern crate cronenberg;
use cronenberg::cron_item::CronItem;
use cronenberg::cron_item::TimeItem::*;
extern crate notify_rust;
use notify_rust::Notification;

mod cron_writer;
mod crontab;
use cron_writer::CronWriter;
use crontab::Crontab;
use clap::App;

fn main() {
    App::new("doomsday")
        .version("0.1.0")
        .about("Notifies about recurring events")
        .author("Ayrat Badykov")
        .get_matches();
}
