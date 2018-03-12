use yaml_rust::{YamlLoader, YamlEmitter};
use crontab::Crontab;
use cronenberg::CronItem;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::io::prelude::*;

static NOTIFICATION_COMMAND: &'static str = "doomsday -m";

pub fn parse_notifications_from_yaml(path: &str) -> Crontab {
    let yaml = read_yaml(path);

    parse_yaml(yaml)
}

fn read_yaml(path: &str) -> String {
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);

    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    contents
}

fn parse_yaml(string: String) -> Crontab {
    let yaml = YamlLoader::load_from_str(string.as_str()).unwrap();

    let mut items = vec!();
    for notification in yaml[0]["schedule"].as_vec().unwrap() {
        let cron_item = CronItem::from_str(notification[0].as_str().unwrap()).unwrap();
        let message = notification[1].as_str().unwrap();

        items.push(CronItem{ command: build_command(message), ..cron_item});
    }

    Crontab { items }
}

fn build_command(message: &str) -> String {
    format!("{} {}", NOTIFICATION_COMMAND, message)
}

mod tests {
    use std::str::FromStr;
    use super::parse_notifications_from_yaml;
    use crontab::Crontab;
    use cronenberg::cron_item::CronItem;

    #[test]
    fn parse_simple_file() {
        let parsed_crontab = parse_notifications_from_yaml("./tests/yml_examples/simple.yml");
        let expected_crontab = Crontab {
            items: vec!(
                CronItem::from_str("* * 5-7 1,2,5 8,1 doomsday -m hello").unwrap(),
                CronItem::from_str("* * * * * doomsday -m goodbye").unwrap()
            )
        };

        assert_eq!(parsed_crontab, expected_crontab);
    }
}
