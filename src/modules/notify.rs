use anyhow::Result;
use colored::Colorize;
use humantime::format_duration;
use notify_rust::Notification;
use tokio::time::Duration;

pub async fn notify_connection_ok() -> Result<()> {
    println!("[{}] Connection Ok", "WARNING!".green());
    if let Err(e) = Notification::new()
        .summary("Statutil")
        .body("Connection Ok")
        .show_async()
        .await
    {
        eprintln!("[{}] Notification Error: {}", "ERROR".red(), e);
    };
    Ok(())
}

pub async fn notify_bad_connection(ping: Duration) -> Result<()> {
    println!(
        "[{}] Bad Connection ({})",
        "WARNING!".yellow(),
        format_duration(ping)
    );
    if let Err(e) = Notification::new()
        .summary("Statutil")
        .body(&format!("Bad Connection ({})", format_duration(ping)))
        .show_async()
        .await
    {
        eprintln!("[{}] Notification Error: {}", "ERROR".red(), e);
    };
    Ok(())
}

pub async fn notify_no_connection() -> Result<()> {
    println!("[{}] No Connection", "WARNING!".red());
    if let Err(e) = Notification::new()
        .summary("Statutil")
        .body("NO CONNECTION")
        .show_async()
        .await
    {
        eprintln!("[{}] Notification Error: {}", "ERROR".red(), e);
    };
    Ok(())
}
