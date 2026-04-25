use anyhow::Context;
use argon2::{Argon2, PasswordHash, password_hash::{self, SaltString}};
use axum::{Extension, Json, Router, routing::post};
use serde::{Serialize, Deserialize};
use crate::api::{Error, Result, ResultExt};
use super::types::extractor::AuthUser;

use crate::api::AppContext;


pub fn router() -> Router {
    Router::new()
        .route("", post(create_user))
}

#[derive(Serialize, Deserialize)]
struct UserBody<T> {
    user: T
}

#[derive(Serialize, Deserialize)]
struct User {
    email: String,
    token: String,
    username: String
}

#[derive(Serialize, Deserialize)]
struct NewUser {
    email: String,
    username: String,
    password: String
}

async fn create_user(
    ctx: Extension<AppContext>,
    Json(req): Json<UserBody<NewUser>>
) -> Result<Json<UserBody<User>>> {
    let password_hash = hash_password(req.user.password).await?;
    
    let user_query = format!(
        r#"insert into "user" (username, email, password_hash) values ({username}, {email}, {password_hash}) returning user_id"#,
        username = req.user.username,
        email = req.user.email,
        password_hash = password_hash
    );

    let user_id = sqlx::query_scalar(&user_query)
        .fetch_one(&ctx.db)
        .await
        .on_constraint("user_username_key", |_| {
            Error::unprocessable_content([("username", "username taken")])
        })
        .on_constraint("user_email_key", |_| {
            Error::unprocessable_content([("email", "email taken")])
        })?;

    Ok(Json(UserBody { 
        user: User {
            email: req.user.email,
            token: AuthUser { user_id }.to_jwt(&ctx),
            username: req.user.username,
        } 
    }))
}

async fn hash_password(password: String) -> Result<String> {
    Ok(
        tokio::task::spawn_blocking(move || -> Result<String> {
            let salt_string = SaltString::generate(
                &mut password_hash::rand_core::OsRng
            );
            let hash = PasswordHash::generate(
                Argon2::default(), 
                password, 
                &salt_string
            );
            Ok(
                hash
                    .map_err(|e| anyhow::anyhow!("failed to generated password hash: {}", e))?
                    .to_string()
            )
    })
    .await
    .context("panic in generating context hash")??)
}