use notify_rust::Notification;

fn notify(message: &str) {
    Notification::new()
        .summary("Doomsday")
        .body(message)
        .show()
        .unwrap();
}
