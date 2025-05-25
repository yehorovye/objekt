use std::{collections::HashMap, env, sync::Arc};

use actix_web::{App, HttpServer, web::Data};
use anyhow::Result;
use providers::fs::FileSystemProvider;
use structs::user::User;
use tokio::sync::Mutex;
use tracing::info;

mod guards;
mod providers;
mod routes;
mod structs;
mod utils;

const DEFAULT_PORT: u16 = 8080;

pub struct AppState {
    users: Mutex<HashMap<String, User>>,
    provider: Arc<FileSystemProvider>,
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

    let shared_data = Data::new(AppState {
        users: Mutex::new(HashMap::new()),
        provider: Arc::new(FileSystemProvider::new("./store".into()).await?), // dont forget to
                                                                              // ignore this path
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(routes::routes)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await?;

    Ok(())
}
