use std::env;

use reqwest;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub struct App {
    pub https: reqwest::Client,
    pub db: Pool<Postgres>,
    pub pterodactyl: pterodactyl_api::client::Client,
}

impl App {
    pub async fn new() -> Self {
        Self {
            https: reqwest::Client::new(),

            db: PgPoolOptions::new()
                .connect("postgres://postgres:postgres@localhost/postgres")
                .await.expect("Unknown error occurred while connecting to DB"),

            pterodactyl: pterodactyl_api::client::ClientBuilder::new(
                env::var("PTERODACTYL_URL").expect("Missing Required Env Var PTERODACTYL_URL"),
                env::var("PTERODACTYL_APIKEY").expect("Missing Required Env Var PTERODACTYL_APIKEY"),
            ).build(),
        }
    }
}