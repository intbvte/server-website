use rocket::form::Form;
use std::env;
use serde_json::json;

use rocket::State;

use crate::app::App;
use crate::Whitelist;

pub async fn minecraft_whitelist(app: &State<App>, whitelist_data: &Form<Whitelist>) {
    let response = app.https.put(format!("{}/playerlists/whitelist/", &app.exaroton_url))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", &app.exaroton_key))
        .json(&json!({
            "entries": [&whitelist_data.username]
        }))
        .send()
        .await;

    match response {
        Ok(_) => (),
        Err(err) => { error!("{} \n {}", format!("Error while whitelisting {}", whitelist_data.username), err) }
    }
}

pub async fn minecraft_whitelist_remove(app: &State<App>, username: &str) {
    let response = app.https.delete(format!("{}/playerlists/whitelist", &app.exaroton_url))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", &app.exaroton_key))
        .json(&json!({
            "entries": [&username]
        }))
        .send()
        .await;

    match response {
        Ok(_) => (),
        Err(err) => { error!("{} \n {}", format!("Error while un-whitelisting {}", username), err) }
    }
}