use anyhow::{Error, Result};
use std::net::IpAddr;
use surge_ping::ping;
use tokio::net::lookup_host;
use tokio::time::Duration;

pub async fn get_ip_by_host(host: &str) -> Result<Option<IpAddr>> {
    match lookup_host((host, 0)).await.into_iter().next() {
        Some(mut ip_iter) => match ip_iter.next() {
            Some(ip) => Ok(Some(ip.ip())),
            None => Ok(None),
        },
        None => Ok(None),
    }
}

pub async fn ping_ip(ip: IpAddr) -> Result<Duration, Error> {
    match ping(ip, &[1, 2, 3, 4, 5, 6, 7, 8]).await {
        Ok((_packet, duration)) => Ok(duration),
        Err(e) => Err(e.into()),
    }
}
