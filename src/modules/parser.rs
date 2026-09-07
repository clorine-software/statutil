use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, default_value_t = String::from("clorine.ru"))]
    pub host: String,

    #[arg(short, long, default_value_t = 64)]
    pub bad_ping_border: u64,

    #[arg(short, long, default_value_t = 1000)]
    pub loop_interval: u64,
}

pub async fn parse_args() -> Result<Args> {
    Ok(Args::parse())
}
