use std::{env, sync::Arc};

use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use anyhow::Result;
use providers::{fs::FileSystemProvider, memory::MemoryProvider};
use serde_json::Value;
use structs::user::User;
use tracing::info;

mod guards;
mod providers;
mod routes;
mod structs;

const DEFAULT_PORT: u16 = 8080;

pub struct AppState {
    users: Arc<FileSystemProvider<User>>,
    provider: Arc<MemoryProvider<Value>>,
}

#[actix_web::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    info!("Initialized tracing_subscriber");

    // todo: select provider from env

    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(DEFAULT_PORT);

    // Users are stored in the same way cache is
    // Warning: When using fs provider, remember to ignore the path
    let shared_data = Data::new(AppState {
        users: Arc::new(FileSystemProvider::new("./users".into()).await?),
        provider: Arc::new(MemoryProvider::new(50)),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(shared_data.clone())
            .configure(routes::routes)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
