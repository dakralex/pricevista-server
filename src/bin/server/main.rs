use axum::routing::get;
use axum::Router;
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use pricevista::handler;
use sibyl::{Environment, SessionPool};
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(name = "pvserve", author, version, about, long_about = None)]
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
            default_value = "0.0.0.0",
            help = "Hostname or IP address to listen on"
        )]
        host: String,
        #[arg(
            env,
            long,
            short = 'P',
            default_value_t = 8000,
            help = "Port number to listen on"
        )]
        port: u16,
        #[arg(env, long, help = "TNS alias for the database to connect to")]
        db_name: String,
        #[arg(env, long, help = "Username for the database connection")]
        db_user: String,
        #[arg(env, long, help = "Password for the database connection")]
        db_pass: String,
        #[arg(
            env,
            long,
            default_value_t = 10,
            help = "Maximum number of database connections"
        )]
        db_max_conn: usize,
    },
}

lazy_static! {
    pub static ref ORACLE: Environment = sibyl::env().expect("Oracle OCI environment");
}

pub struct AppState<'a> {
    #[allow(unused)]
    db: SessionPool<'a>,
}

#[tokio::main]
async fn main() {
    match dotenvy::dotenv() {
        Ok(path) => println!("Environment file successfully read from {}", path.display()),
        Err(err) => println!("Could not load environment file: {err}"),
    }

    let args = Cli::parse();

    let oracle = &ORACLE;

    match args.command {
        Commands::Serve {
            host,
            port,
            db_name,
            db_user,
            db_pass,
            db_max_conn,
        } => {
            let pool = oracle
                .create_session_pool(&db_name, &db_user, &db_pass, 1, 1, db_max_conn)
                .await;
            let pool = match pool {
                Ok(pool) => {
                    println!("Connected to the database {} successfully.", db_name);
                    pool
                }
                Err(err) => {
                    println!("Could not connect to the database {}: {:?}", db_name, err);
                    std::process::exit(1);
                }
            };

            let app_state = Arc::new(AppState { db: pool });
            let app = Router::new()
                .route("/api/health", get(handler::health_handler))
                .with_state(app_state);

            let bind_address = format!("{}:{}", host, port);
            let listener = tokio::net::TcpListener::bind(&bind_address).await.unwrap();

            println!("Running server at {}...", bind_address);

            axum::serve(listener, app).await.unwrap();
        }
    }
}
