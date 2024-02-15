use clap::{Parser, Subcommand};
use pricevista::fetcher::{fetch_billa, fetch_mpreis, fetch_spar, FetchSourceType};
use pricevista::importer::ImportSourceType;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Could not read environment file.");
    let args = ManagerCli::parse();

    match args.command {
        Commands::Fetch { source, .. } => {
            match source {
                FetchSourceType::Billa => {
                    let response = fetch_billa();
                    println!("{:?}", response.await.unwrap());
                }
                FetchSourceType::Mpreis => {
                    let response = fetch_mpreis();
                    response.await.unwrap();
                }
                FetchSourceType::Spar => {
                    let response = fetch_spar();
                    println!("{:?}", response.await.unwrap());
                }
                _ => todo!("This fetch type has not been implemented yet."),
            };
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
        source: FetchSourceType,
    },
}
