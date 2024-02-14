use clap::{Args, Parser, Subcommand};
use pricevista::fetcher::{fetch_billa, FetchSourceType};
use pricevista::importer::ImportSourceType;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Could not read environment file.");
    let args = ManagerCli::parse();

    match args.command {
        Commands::Fetch(_) => {
            let response = fetch_billa();

            println!("{:?}", response.await.unwrap());
        }
        _ => todo!("This argument has not been implemented yet."),
    };
}

#[derive(Parser)]
#[command(author, version, long_about = None)]
#[command(about = "Manager for the PriceVista server")]
struct ManagerCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the application
    Init(InitArgs),
    /// Import data from various file formats
    Import(ImportArgs),
    /// Update data through fetching API endpoints
    Fetch(FetchArgs),
}

#[derive(Args)]
struct InitArgs {}

#[derive(Args)]
struct ImportArgs {
    #[arg(long, help = "Specify the source of the import file")]
    source: ImportSourceType,
    #[arg(required = true)]
    files: Vec<std::path::PathBuf>,
}

#[derive(Args)]
struct FetchArgs {
    #[arg(long, help = "Specify the source to fetch from")]
    source: FetchSourceType,
}
