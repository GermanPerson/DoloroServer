pub mod models;
pub mod schema;
mod api;

use std::error::Error;
use self::models::*;

use diesel::sqlite::{Sqlite, SqliteConnection};
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

const MIGRATIONS : EmbeddedMigrations = embed_migrations!();

use poem::{listener::TcpListener, Route, Server};
use poem_openapi::{OpenApi, OpenApiService};

extern crate dotenv;

use dotenv::dotenv;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    println!("Hello, world!");

    let conn = &mut establish_connection();
    run_migrations(conn).unwrap();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "poem=debug");
    }
    tracing_subscriber::fmt::init();

    let api_service =
        OpenApiService::new(api::pishock::Api, "Users", "1.0").server("http://localhost:3000/api");
    let ui = api_service.swagger_ui();

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(Route::new().nest("/api", api_service).nest("/", ui))
        .await
}

pub fn establish_connection() -> SqliteConnection {
    

    SqliteConnection::establish("database.db")
        .expect("Error opening the database at database.db")
}

fn run_migrations(connection: &mut impl MigrationHarness<Sqlite>) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}
