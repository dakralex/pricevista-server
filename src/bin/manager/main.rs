use clap::{Parser, Subcommand};
use pricevista::importer::ImportSourceType;
use pricevista::providers::FetchSource;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Could not read environment file.");
    let args = ManagerCli::parse();

    match args.command {
        Commands::Fetch { source, .. } => {}
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
    Init,
    /// Import data from various file formats
    Import {
        #[arg(long, help = "Specify the source of the import file")]
        source: ImportSourceType,
        #[arg(required = true)]
        files: Vec<std::path::PathBuf>,
    },
    /// Update data through fetching API endpoints
    Fetch {
        #[arg(long, help = "Specify the source to fetch from")]
        source: FetchSource,
    },
}
