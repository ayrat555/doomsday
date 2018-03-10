use yaml_rust::{YamlLoader, YamlEmitter};
use crontab::Crontab;
use cronenberg::cron_item::CronItem;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::fs::File;

pub fn parse_yaml(path: String) -> Crontab {
    let file = BufReader::new(File::open("read_file.rs").unwrap());
    let items = file.lines().map(|line| { CronItem::from_str(line.unwrap().as_str()).unwrap() }).collect();

    Crontab { items }
}

mod tests {
    use super::parse_yaml;

    #[test]
    fn parse_simple_file() {
        result =
    }
}
