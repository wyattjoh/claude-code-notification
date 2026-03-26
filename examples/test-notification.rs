use notify_rust::Notification;

fn main() {
    println!("Testing notification system...");

    match Notification::new()
        .summary("Test Notification")
        .body("This is a test notification from claude-code-notification")
        .show()
    {
        Ok(_) => println!("✅ Notification sent successfully"),
        Err(e) => eprintln!("❌ Notification failed: {}", e),
    }
}
