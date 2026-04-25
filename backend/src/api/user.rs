use anyhow::Context;
use argon2::{Argon2, PasswordHash, password_hash::{self, SaltString}};
use axum::{Extension, Json, Router, routing::post};
use serde::{Serialize, Deserialize};
use crate::api::{Error, Result, ResultExt};
use super::types::extractor::AuthUser;

use crate::api::AppContext;


pub fn router() -> Router {
    Router::new()
        .route("/create", post(create_user))
        .route("/login", post(login_user))
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

#[derive(serde::Deserialize)]
struct LoginUser {
    email: String,
    password: String
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
    

    let user_id = sqlx::query_scalar!(
            r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
            req.user.username,
            req.user.email,
            password_hash
        )
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

async fn login_user(
    ctx: Extension<AppContext>,
    Json(req): Json<UserBody<LoginUser>>
) -> Result<Json<UserBody<User>>> {

    let user = sqlx::query!(
            r#"select user_id, email, username, password_hash from "user" where email = $1"#,
            req.user.email
        )
        .fetch_optional(&ctx.db)
        .await?
        .ok_or(Error::unprocessable_content([("email", "does not exist")]))?;

    verify_password(req.user.password, user.password_hash).await?;

    Ok(Json(UserBody { 
        user: User {
            email: user.email,
            token: AuthUser {
                user_id: user.user_id
            }.to_jwt(&ctx),
            username: user.username,
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

async fn verify_password(password: String, password_hash: String) -> Result<()> {
    Ok(tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!("invalid password has: {}", e))?;

        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => Error::Unauthorized,
                _ => anyhow::anyhow!("failed to verify password hash: {}", e).into()
            }) 
    })
    .await
    .context("panic in verifying password has")??)
}