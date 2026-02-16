use anyhow::Context;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
#[allow(unused)]
pub async fn print(s: &str) -> anyhow::Result<()> {
    tokio::io::stdout()
        .write_all(format!("{}\n", s).as_bytes())
        .await
        .with_context(|| "stdout error")
}

#[allow(unused)]
pub async fn input() -> anyhow::Result<String> {
    let mut reader = BufReader::new(io::stdin());
    let mut result = String::new();
    reader.read_line(&mut result).await?;
    Ok(result.trim().to_string())
}
