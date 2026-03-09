use clap::{Parser, Subcommand};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use api::{build_app, AppState};
use sqlx::postgres::PgPoolOptions;

#[derive(Parser, Debug)]
#[command(name = "serve")]
#[command(version)]
#[command(about = "CLI for running the app")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Server {
        #[arg(long, default_value_t = 3000)]
        port: u16,

        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Server { host, port } => {
            run_server(host, port).await?;
        }
    }

    Ok(())
}

async fn run_server(host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let pg_url = "postgres://or:or@localhost:5432/or";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(pg_url)
        .await
        .unwrap();
    let state = AppState { db: pool };
    let app = build_app(state);
    let addr: SocketAddr = format!("{host}:{port}").parse()?;
    let listener = TcpListener::bind(addr).await?;

    println!("server listening on http://{addr}");

    axum::serve(listener, app).await?;

    Ok(())
}