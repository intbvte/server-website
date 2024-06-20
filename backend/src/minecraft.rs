use std::env;

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

use crate::app::App;
use crate::{minecraft, Whitelist};

async fn minecraft_whitelist(app: &State<App>, whitelist_data: Json<Whitelist>) -> (Status, &'static str) {
    run_command(
        app,
        format!("whitelist add {}", whitelist_data.username),
        "User was whitelisted successfully",
        format!("A unknown error occurred while whitelisting user {}", whitelist_data.username),
    ).await
}

async fn minecraft_whitelist_remove(app: &State<App>, whitelist_data: Json<Whitelist>) -> (Status, &'static str) {
    run_command(
        app,
        format!("whitelist remove {}", whitelist_data.username),
        "User was un-whitelisted successfully",
        format!("A unknown error occurred while un-whitelisting user {}", whitelist_data.username),
    ).await
}

pub async fn run_command(app: &State<App>, command: String, success_message: &'static str, error_message: String) -> (Status, &'static str) {
    let server_id = env::var("PTERODACTYL_SERVER_ID").expect("Missing Required Env Var PTERODACTYL_SERVER_ID");

    let response = app.pterodactyl.get_server(server_id)
        .send_command(command)
        .await;

    match response {
        Ok(_) => (Status::Ok, success_message),
        Err(err) => {
            error!("{} \n {}", error_message, err);

            (Status::InternalServerError, "A unknown error occurred while processing this request")
        }
    }
}