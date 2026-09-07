use crate::modules::{self, net, parser};
use anyhow::Result;
use tokio::time::{Duration, sleep};

#[derive(PartialEq)]
enum LastPingStatus {
    Ok,
    Bad,
    NoConnection,
}

pub async fn main() -> Result<()> {
    colored::control::set_override(true);

    let mut last_ping: LastPingStatus = LastPingStatus::Ok;

    let args = parser::parse_args().await?;

    loop {
        let addr = match modules::net::get_ip_by_host(&args.host).await? {
            Some(ip) => ip,
            None => {
                modules::notify::notify_no_connection().await?;
                continue;
            }
        };
        match net::ping_ip(addr).await {
            Ok(ping) => {
                if ping > Duration::from_millis(args.bad_ping_border) {
                    modules::notify::notify_bad_connection(ping).await?;
                    last_ping = LastPingStatus::Bad;
                } else {
                    if last_ping != LastPingStatus::Ok {
                        modules::notify::notify_connection_ok().await?;
                    }
                    last_ping = LastPingStatus::Ok;
                }
            }
            Err(_e) => {
                modules::notify::notify_no_connection().await?;
                last_ping = LastPingStatus::NoConnection;
            }
        };

        sleep(Duration::from_millis(args.loop_interval)).await;
    }
}
