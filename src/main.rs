extern crate chrono;
extern crate notify_rust;

mod org_parser;
mod notification;
use schedule::Agenda;

fn main() {
    let mut a = Agenda::new();

    a.add(|| {
        Notification::new()
            .summary("Doomsday")
            .body("Hey")
            .show().unwrap();
    }).schedule("* * * * * *").unwrap();
}
