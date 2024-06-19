use std::env;
use reqwest::Client;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub struct App {
    pub https: Client,
    pub database: Pool<Postgres>,
    pub pterodactyl: pterodactyl_api::client::Client
}

impl App {
    pub async fn new() -> Self {
        Self {
            https: Client::new(),
            database: PgPoolOptions::new()
                .connect("postgres://postgres:password@localhost/test")
                .await.expect("Error occurred with db"),
            pterodactyl: pterodactyl_api::client::ClientBuilder::new(
                    env::var("PTERODACTYL_URL").expect("Missing Required Env Var PTERODACTYL_URL"),
                    env::var("PTERODACTYL_APIKEY").expect("Missing Required Env Var PTERODACTYL_APIKEY")
                ).build()
        }
    }
}