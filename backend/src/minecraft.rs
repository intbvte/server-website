use std::env;
use rocket::form::Form;

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

pub async fn minecraft_whitelist_remove(app: &State<App>, whitelist_data: &Form<Whitelist>) {
    run_command(
        app,
        format!("whitelist remove {}", whitelist_data.username),
        format!("A unknown error occurred while un-whitelisting user {}", whitelist_data.username),
    ).await
}

async fn run_command(app: &State<App>, command: String, error_message: String) {
    let server_id = env::var("PTERODACTYL_SERVER_ID").expect("Missing Required Env Var PTERODACTYL_SERVER_ID");

    let response = app.pterodactyl.get_server(server_id)
        .send_command(command)
        .await;

    match response {
        Ok(_) => (),
        Err(err) => { error!("{} \n {}", error_message, err) }
    }
}