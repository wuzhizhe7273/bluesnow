use bluesnow_result::Result;
use cli::Cli;
#[tokio::main]
async fn main() -> Result<()> {
    Cli::run().await
}
