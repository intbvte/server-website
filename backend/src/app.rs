use reqwest;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::collections::HashMap;
use std::env;
use std::sync::{Arc, RwLock};
use std::time::Instant;

pub struct App {
    pub https: reqwest::Client,
    pub db: Pool<Postgres>,
    pub cache: Arc<RwLock<HashMap<(&'static str, u64), (String, Instant)>>>,
    pub exaroton_key: String,
    pub server_id: String,
    pub exaroton_url: String,
}

impl App {
    pub async fn new() -> Self {
        let server_id = env::var("EXAROTON_SERVER_ID").expect("Missing EXAROTON_SERVER_ID");
        let exaroton_key = env::var("EXAROTON_API_KEY").expect("Missing EXAROTON_API_KEY");
        let exaroton_url = format!(
            "https://api.exaroton.com/v1/servers/{}",
            server_id
        );

        Self {
            https: reqwest::Client::new(),

            db: PgPoolOptions::new()
                .connect(&env::var("DATABASE_URL").expect("Missing Required Env Var DATABASE_URL"))
                .await.expect("Unknown error occurred while connecting to DB"),

            cache: Arc::new(RwLock::new(HashMap::new())),
            exaroton_key,
            server_id,
            exaroton_url,
        }
    }
}