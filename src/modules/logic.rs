use crate::modules::{self, net, notify, parser};
use anyhow::Result;
use tokio::{sync::Mutex, time::{Duration, sleep}};
use std::sync::Arc;

#[derive(PartialEq)]
enum LastPingStatus {
    Ok,
    Bad,
    NoConnection,
}

pub async fn main() -> Result<()> {
    colored::control::set_override(true);

    let last_ping_arc: Arc<Mutex<LastPingStatus>> = Arc::new(Mutex::new(LastPingStatus::Ok));

    let args = parser::parse_args().await?;

    loop {
        let args_clone = args.clone();
        let last_ping_mutex = Arc::clone(&last_ping_arc);
        tokio::spawn(async move {
            if let Err(e) = ping_logic(&args_clone, &last_ping_mutex).await { eprintln!("Ping logic error: {}", e) };
        });

        sleep(Duration::from_millis(args.loop_interval)).await;
    }
}

async fn ping_logic(args: &parser::Args, last_ping_mutex: &Mutex<LastPingStatus>) -> Result<()> {
    let addr = match modules::net::get_ip_by_host(&args.host).await? {
        Some(ip) => ip,
        None => {
            notify::notify_no_connection(&args, notify::NoConnectionLevel::Dns).await?;
            return Ok(())
        }
    };

    match net::ping_ip(addr).await {
        Ok(ping) => {
            if ping > Duration::from_millis(args.bad_ping_border) {
                notify::notify_bad_connection(ping, &args).await?;
                let mut last_ping = last_ping_mutex.lock().await;
                *last_ping = LastPingStatus::Bad;
            } else {
                let mut last_ping = last_ping_mutex.lock().await;
                if *last_ping != LastPingStatus::Ok {
                    notify::notify_connection_ok(&args).await?;
                    *last_ping = LastPingStatus::Ok;
                }
            }
        }
        Err(_e) => {
            notify::notify_no_connection(&args, notify::NoConnectionLevel::Ping).await?;
            let mut last_ping = last_ping_mutex.lock().await;
            *last_ping = LastPingStatus::NoConnection;
        }
    };

    Ok(())
}
