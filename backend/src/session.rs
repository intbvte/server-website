use chrono::{Duration, Local};
use rand::RngCore;
use rocket::{State, time};
use rocket::http::{Cookie, SameSite};
use sqlx::query;

use crate::app::App;
use crate::DiscordCallback;

pub async fn generate_session<'a>(app: &State<App>, access_token: &str, refresh_token: &str, token_expiry: i64) -> Cookie<'a> {
    let callback = app.https.get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", access_token))
        .send()
        .await
        .unwrap()
        .json::<DiscordCallback>()
        .await
        .unwrap();

    generate_session_with_callback(app, callback, access_token, refresh_token, token_expiry).await
}

pub async fn generate_session_with_callback<'a>(app: &State<App>, discord_callback: DiscordCallback, access_token: &str, refresh_token: &str, token_expiry: i64) -> Cookie<'a> {
    let secs: i64 = token_expiry.try_into().unwrap();

    let max_age = Local::now().naive_local() + Duration::seconds(secs);

    let mut u128_pool = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut u128_pool);

    let session_id = u128::from_le_bytes(u128_pool).to_string();

    let user_id = discord_callback.id.parse::<i64>().expect("Failed to read user.id as a i64");

    query!("INSERT INTO sessions (user_id, session_id, expires_at, access_token, refresh_token)
            VALUES ($1, $2, $3, $4, $5)",
        user_id, session_id, max_age.and_utc(), access_token, refresh_token)
        .execute(&app.db)
        .await
        .unwrap();

    Cookie::build(("session_id", session_id.to_owned()))
        .same_site(SameSite::Lax)
        .max_age(time::Duration::seconds(secs))
        .build()
}