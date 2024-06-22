#[macro_use]
extern crate rocket;

use std::env;
use chrono::{DateTime, Utc};

use dotenvy::dotenv;
use rocket::{Request, State};
use rocket::http::{CookieJar, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::response::Redirect;
use rocket::serde::json::Json;
use rocket_oauth2::{OAuth2, TokenResponse};
use serde::Deserialize;
use sqlx::query;

use crate::app::App;
use crate::errors::ApiError;

mod minecraft;
mod app;
mod errors;
mod session_manager;

struct Discord;

#[derive(Deserialize)]
struct Whitelist {
    username: String,
}

#[derive(Deserialize)]
struct DiscordCallback {
    id: String,
    username: String,
}

#[derive(Deserialize)]
struct DiscordAccessTokenResponse {
    access_token: String,
    _token_type: String,
    expires_in: i64,
    refresh_token: String,
    _scope: String,
}

#[derive(Deserialize)]
struct MinecraftUsernameToUUID {
    name: String,
    id: String
}

pub struct User {
    pub discord_id: i64,
    pub discord_username: String,
    pub minecraft_uuid: Option<String>,
    pub created_at: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,
    pub is_admin: bool
}

pub struct Session {
    pub user: User,
    pub session_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub expired: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Session {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let app = request.rocket().state::<App>().unwrap();

        let session_cookie = request
            .cookies()
            .get_private("session_id");

        let cookie = match session_cookie {
            Some(token) => token,
            None => {
                return Outcome::Error((
                    Status::BadRequest,
                    String::from("Session Id cookie is missing"),
                ))
            },
        };

        let session_id = cookie.value();

        let session = query!("SELECT * FROM sessions WHERE session_id = $1", session_id)
            .fetch_optional(&app.db)
            .await
            .unwrap();

        if let Some(session) = session {
            let user = query!("SELECT * FROM users WHERE discord_id = $1", session.user_id)
                .fetch_optional(&app.db)
                .await
                .unwrap();

            if let Some(user) = user {
                return Outcome::Success(Session {
                    user: User {
                        discord_id: user.discord_id,
                        discord_username: user.discord_username,
                        minecraft_uuid: user.minecraft_uuid,
                        created_at: user.created_at,
                        last_updated: user.last_updated,
                        is_admin: user.is_admin
                    },
                    session_id: session.session_id,
                    access_token: session.access_token,
                    refresh_token: session.refresh_token,
                    expires_at: session.expires_at,
                    expired: session.expired
                })
            }
        }

        Outcome::Error((Status::BadRequest, "A error occurred with that request".to_string()))
    }
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let app: App = App::new().await;

    sqlx::migrate!().run(&app.db).await.expect("Failed to apply migrations :(");

    rocket::build()
        .manage(app)
        .mount("/backend/", routes![discord_login, discord_logout, discord_callback, minecraft_username_change])
        .attach(OAuth2::<Discord>::fairing("discord"))
}

#[get("/login/discord")]
async fn discord_login(app: &State<App>, oauth2: OAuth2<Discord>, cookies: &CookieJar<'_>) -> Redirect {
    let session_cookie = cookies.get_private("session_id");

    if let Some(cookie) = session_cookie {
        let session_id = cookie.value();

        let session = query!("SELECT * FROM sessions WHERE session_id = $1 AND expired = FALSE AND expires_at > NOW()", session_id)
            .fetch_optional(&app.db)
            .await
            .unwrap();

        if let Some(session) = session {
            let req = app.https.post("https://discord.com/api/oauth2/token")
                .header("Content-Type", "application/x-www-form-urlencoded")
                .basic_auth(
                    env::var("DISCORD_CLIENT_ID").expect("Missing client id"),
                    Some(env::var("DISCORD_CLIENT_SECRET").expect("Missing client secret")),
                )
                .body(format!("grant_type=refresh_token&refresh_token={}", session.refresh_token))
                .send()
                .await
                .unwrap()
                .json::<DiscordAccessTokenResponse>()
                .await
                .unwrap();

            query!("UPDATE sessions SET expired = true WHERE session_id = $1", session_id)
                .execute(&app.db)
                .await
                .unwrap();

            let session_cookie = session_manager::generate_session(app, &req.access_token, &req.refresh_token, req.expires_in).await;
            cookies.add_private(session_cookie);

            return Redirect::to("/");
        }
    };

    oauth2.get_redirect(cookies, &["identify"]).unwrap()
}

#[get("/logout/discord")]
async fn discord_logout(app: &State<App>, cookies: &CookieJar<'_>) -> Redirect {
    let session_cookie = cookies.get_private("session_id");

    if let Some(cookie) = session_cookie {
        let session_id = cookie.value();

        let session = query!("SELECT * FROM sessions WHERE session_id = $1 AND expired = FALSE AND expires_at > NOW()", session_id)
            .fetch_optional(&app.db)
            .await
            .unwrap()
            .unwrap();

        query!("UPDATE sessions SET expired = true WHERE session_id = $1", session_id)
            .execute(&app.db)
            .await
            .unwrap();

        session_manager::revoke_discord_token(app, session.access_token).await;
        session_manager::revoke_discord_token(app, session.refresh_token).await;

        cookies.remove_private("session_id");
    }

    Redirect::to("/")
}

#[get("/auth/discord")]
async fn discord_callback(app: &State<App>, token: TokenResponse<Discord>, cookies: &CookieJar<'_>) -> Result<Redirect, ApiError> {
    let Some(secs) = token.expires_in() else {
        return Err(ApiError::OptionError);
    };

    let user = app.https.get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", token.access_token()))
        .send()
        .await
        .unwrap()
        .json::<DiscordCallback>()
        .await
        .unwrap();

    let user_id = match user.id.parse::<i64>() {
        Ok(i) => i,
        Err(err) => return Err(ApiError::ParseStringAsIntError(err))
    };

    query!("INSERT INTO users (discord_id, discord_username)
            VALUES ($1, $2)
            ON CONFLICT (discord_id) DO NOTHING;", user_id, user.username)
        .execute(&app.db)
        .await
        .unwrap();

    let session_cookie = session_manager::generate_session_with_callback(app, user, token.access_token(), token.refresh_token().unwrap(), secs).await;
    cookies.add_private(session_cookie);

    //fixme change redirect location
    Ok(Redirect::to("/"))
}

#[post("/minecraft/username/change", format = "application/json", data = "<whitelist_data>")]
async fn minecraft_username_change(app: &State<App>, session: Session, whitelist_data: Json<Whitelist>) -> Result<(), ApiError> {
    let user_profile = app.https.get(format!("https://api.mojang.com/users/profiles/minecraft/{}", whitelist_data.username))
        .send()
        .await
        .unwrap()
        .json::<MinecraftUsernameToUUID>()
        .await
        .unwrap();
    
    let query = query!("UPDATE users SET minecraft_uuid = $1 WHERE discord_id = $2", user_profile.id, session.user.discord_id)
        .execute(&app.db)
        .await;
    
    match query {
        Ok(_) => {
            minecraft::minecraft_whitelist_remove(app, &whitelist_data).await;
            minecraft::minecraft_whitelist(app, &whitelist_data).await;
            
            Ok(())
        },
        Err(err) => Err(ApiError::SQL(err)),
    }
}