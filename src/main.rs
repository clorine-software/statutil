use std::{net::IpAddr, time::Duration};
use anyhow::{Error, Result};
use notify_rust::Notification;
use tokio::{time::sleep, net::lookup_host};
use surge_ping::ping;
use colored::Colorize;
use humantime::format_duration;

static DNS_ADDR: &str = "clorine.ru";
static LOOP_INTERVAL: u64 = 1000;
static BAD_PING: u64 = 40;

#[derive(PartialEq)]
enum LastPingStatus {
    Ok,
    Bad,
    NoConnection
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut last_ping: LastPingStatus = LastPingStatus::Ok;
    
    loop {
        let addr = match lookup_host((DNS_ADDR,0)).await.into_iter().next() {
            Some(mut ip_iter) => {
                match ip_iter.next() {
                    Some(ip) => {
                        ip.ip()
                    },
                    None => {
                        notify_no_connection().await?;
                        continue;
                    }
                }
            },
            None => {
                notify_no_connection().await?;
                continue;
            },
        };
        match ping_ip(addr).await {
            Ok(ping) => {
                if ping > Duration::from_millis(BAD_PING) {
                    notify_bad_connection(ping).await?;
                    last_ping = LastPingStatus::Bad;
                } else {
                    if last_ping != LastPingStatus::Ok { notify_connection_ok().await?; }
                    last_ping = LastPingStatus::Ok;
                }
            },
            Err(_e) => {
                notify_no_connection().await?;
                last_ping = LastPingStatus::NoConnection;
            },
        };

        sleep(Duration::from_millis(LOOP_INTERVAL)).await;
    }
}

async fn ping_ip(ip: IpAddr) -> Result<Duration, Error> {
    match ping(ip, &[1,2,3,4,5,6,7,8]).await {
        Ok((_packet, duration)) => {
            Ok(duration)
        },
        Err(e) => {
            Err(e.into())
        },
    }
}

async fn notify_connection_ok() -> Result<()> {
    println!("[{}] Connection Ok", "WARNING!".green());
    Notification::new().summary("Statutil").body("Connection Ok").show_async().await?;
    Ok(())
}

async fn notify_bad_connection(ping: Duration) -> Result<()> {
    println!("[{}] Bad Connection ({})", "WARNING!".yellow(), format_duration(ping));
    Notification::new().summary("Statutil").body(&format!("Bad Connection ({})", format_duration(ping))).show_async().await?;
    Ok(())
}

async fn notify_no_connection() -> Result<()> {
    println!("[{}] No Connection", "WARNING!".red());
    Notification::new().summary("Statutil").body("NO CONNECTION").show_async().await?;
    Ok(())
}
