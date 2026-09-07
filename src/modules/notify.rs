use anyhow::Result;
use colored::Colorize;
use humantime::format_duration;
use notify_rust::Notification;
use tokio::time::Duration;

use crate::modules::parser::Args;

pub async fn notify_connection_ok(args: &Args) -> Result<()> {
    println!("[{}] Connection Ok", "WARNING!".green());
    if !args.silent {
        if let Err(e) = Notification::new()
            .summary("Statutil")
            .body("Connection Ok")
            .show()
        {
            eprintln!("[{}] Notification Error: {}", "ERROR".red(), e);
        };
    }
    Ok(())
}

pub async fn notify_bad_connection(ping: Duration, args: &Args) -> Result<()> {
    println!(
        "[{}] Bad Connection ({})",
        "WARNING!".yellow(),
        format_duration(ping)
    );
    if !args.silent {
        if let Err(e) = Notification::new()
            .summary("Statutil")
            .body(&format!("Bad Connection ({})", format_duration(ping)))
            .show()
        {
            eprintln!("[{}] Notification Error: {}", "ERROR".red(), e);
        };
    }
    Ok(())
}

pub async fn notify_no_connection(args: &Args) -> Result<()> {
    println!("[{}] No Connection", "WARNING!".red());
    if !args.silent {
        if let Err(e) = Notification::new()
            .summary("Statutil")
            .body("NO CONNECTION")
            .show()
        {
            eprintln!("[{}] Notification Error: {}", "ERROR".red(), e);
        };
    }
    Ok(())
}
