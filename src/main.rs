use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "pricevista", author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Serve {
        #[arg(
            env,
            long,
            short = 'H',
            help = "Hostname or IP address to listen on",
            default_value = "0.0.0.0"
        )]
        host: String,
        #[arg(
            env,
            long,
            short = 'P',
            help = "Port number to listen on",
            default_value_t = 8000
        )]
        port: u16,
        #[arg(env, long, help = "TNS alias for the database to connect to")]
        db_name: String,
        #[arg(env, long, help = "Username for the database connection")]
        db_user: String,
        #[arg(env, long, help = "Password for the database connection")]
        db_pass: String,
    },
}

async fn health_handler() -> impl IntoResponse {
    const MESSAGE: &str = "As happy and alive as one could be";

    Json(serde_json::json!({
        "status": "success",
        "message": MESSAGE
    }))
}

#[tokio::main]
async fn main() {
    match dotenvy::dotenv() {
        Ok(path) => println!("Environment file successfully read from {}", path.display()),
        Err(e) => println!("Could not load environment file: {e}"),
    }

    let args = Cli::parse();

    match args.command {
        Commands::Serve { host, port, .. } => {
            let bind_address = format!("{}:{}", host, port);
            let app = Router::new().route("/api/health", get(health_handler));

            println!("Running server at {}...", &bind_address);
            let listener = tokio::net::TcpListener::bind(bind_address).await.unwrap();

            axum::serve(listener, app).await.unwrap();
        }
    }
}
