use notify_rust::Notification;

pub fn notify(message: &str) {
    Notification::new()
        .summary("Doomsday")
        .body(message)
        .show()
        .unwrap();
}
