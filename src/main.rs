use anyhow::Result;
use std::env;


#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    dbg!(args);
    Ok(())
}
