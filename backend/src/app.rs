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
}

impl App {
    pub async fn new() -> Self {
        Self {
            https: reqwest::Client::new(),

            db: PgPoolOptions::new()
                .connect(&env::var("DATABASE_URL").expect("Missing Required Env Var DATABASE_URL"))
                .await.expect("Unknown error occurred while connecting to DB"),

            cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}