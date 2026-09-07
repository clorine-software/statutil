use anyhow::Result;

mod modules;
use modules::logic;

#[tokio::main]
async fn main() -> Result<()> {
    logic::main().await?;
    Ok(())
}
