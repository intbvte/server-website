use std::env;

use rocket::http::Status;
use rocket::State;

use crate::app::App;

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