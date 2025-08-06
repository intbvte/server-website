use rocket::form::Form;
use std::env;
use serde_json::json;

use rocket::State;

use crate::app::App;
use crate::Whitelist;

pub async fn minecraft_whitelist(app: &State<App>, whitelist_data: &Form<Whitelist>) {
    run_command(
        app,
        format!("whitelist add {}", whitelist_data.username),
        format!("A unknown error occurred while whitelisting user {}", whitelist_data.username),
    ).await
}

pub async fn minecraft_whitelist_remove(app: &State<App>, username: &str) {
    run_command(
        app,
        format!("whitelist remove {}", username),
        format!("A unknown error occurred while un-whitelisting user {}", username),
    ).await
}

async fn run_command(app: &State<App>, command: String, error_message: String) {
    let server_id = env::var("EXAROTON_SERVER_ID").expect("Missing Required Env Var EXAROTON_SERVER_ID");
    let exaroton_key = env::var("EXAROTON_API_KEY").expect("Missing Required Env Var EXAROTON_API_KEY");

    let response = app.https.post(format!("https://api.exaroton.com/v1/servers/{}/command/", server_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", exaroton_key))
        .json(&json!({
            "command": command
        }))
        .send()
        .await;

    match response {
        Ok(_) => (),
        Err(err) => { error!("{} \n {}", error_message, err) }
    }
}