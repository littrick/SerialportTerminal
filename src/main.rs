use serialport_terminal::run_terminal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run_terminal().await
}
