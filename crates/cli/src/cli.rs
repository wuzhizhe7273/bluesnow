use crate::args::StartArgs;
use crate::db::DbCommands;
use bluesnow_result::Result;
use clap::Parser;
use server::StartParam;

#[derive(clap::Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Start {
        #[command(flatten)]
        arg: StartArgs,
    },
    #[command(subcommand)]
    Db(DbCommands),
}
impl Cli {
    pub async fn run() -> Result<()> {
        dotenvy::dotenv().expect(".env file not found");
        let cli = Cli::parse();
        match cli.command {
            Command::Start { arg } => {
                let param: StartParam = arg.into();
                server::Config::init(param)?;
                let config = server::Config::get()?;
                let app = server::boot::create_app(config).await?;
                server::boot::start(app).await?;
            }
            Command::Db(db_cmd) => {
                db_cmd.exec().await?;
            }
        }
        Ok(())
    }
}
