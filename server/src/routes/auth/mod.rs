use std::env;

use chrono::Utc;
use ciphers::sha256::SHA256;

pub mod user;

macros_utils::routes! {
    load user,

    on "/auth"
}

fn get_secret() -> String {
    env::var("SERVER_SECRET").expect("need SERVER_SECRET env variable")
}

pub fn generate_user_token(user_id: &str, user_secret: &str) -> String {
    let mut hasher = SHA256::new_default();

    let server_secret = get_secret();

    hasher.update(user_id.as_bytes());
    hasher.update(user_secret.as_bytes());
    hasher.update(server_secret.as_bytes());
    hasher.update(Utc::now().to_string().as_bytes());

    let hash = hasher.get_hash();

    hash.iter().map(|byte| format!("{byte:02x}")).collect()
}
