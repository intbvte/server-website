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
    pub pterodactyl: pterodactyl_api::client::Client,
    pub cache: Arc<RwLock<HashMap<u64, (String, Instant)>>>,
}

impl App {
    pub async fn new() -> Self {
        Self {
            https: reqwest::Client::new(),

            db: PgPoolOptions::new()
                .connect(&env::var("DATABASE_URL").expect("Missing Required Env Var DATABASE_URL"))
                .await.expect("Unknown error occurred while connecting to DB"),

            pterodactyl: pterodactyl_api::client::ClientBuilder::new(
                env::var("PTERODACTYL_URL").expect("Missing Required Env Var PTERODACTYL_URL"),
                env::var("PTERODACTYL_APIKEY").expect("Missing Required Env Var PTERODACTYL_APIKEY"),
            ).build(),

            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}