use std::{borrow::Cow, collections::HashMap};

use axum::Json;
use axum::body::{Body};
use axum::http::{HeaderMap, HeaderValue};
use axum::http::header::WWW_AUTHENTICATE;
use axum::{http::StatusCode, response::IntoResponse};
use sqlx::error::DatabaseError;


#[derive(thiserror::Error, Debug)]
pub enum Error {

    /// 401 Unauthorized
    #[error("authentication required")]
    Unauthorized,

    /// 403 Forbidden
    #[error("unable to perform action")]
    Forbidden,

    /// 404 Not Found
    #[error("request not found")]
    NotFound,

    /// 422 Unprocessable Content
    #[error("error in request body")]
    UnprocessableContent {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>
    },

    /// 500 Internal Server Error
    #[error("error occurred in the database")]
    Sqlx(#[from] sqlx::Error),

    /// 500 Internal Server Error
    #[error("an internal server error ocurred")]
    Anyhow(#[from] anyhow::Error)
}

impl Error {

    pub fn unprocessable_content<K, V>(errors: impl IntoIterator<Item = (K,V)>) -> Self
    where 
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>
    {
        let mut error_map = HashMap::new();

        for (k, v) in errors {
            error_map
                .entry(k.into())
                .or_insert_with(Vec::new)
                .push(v.into());
        }

        Self::UnprocessableContent { errors: error_map }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableContent { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Sqlx(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Anyhow(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

impl IntoResponse for Error {

    fn into_response(self) -> axum::response::Response<Body> {
        match self {
            Self::UnprocessableContent { errors } => {
                #[derive(serde::Serialize)]
                struct Errors {
                    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>
                }

                return (StatusCode::UNPROCESSABLE_ENTITY, Json(Errors {errors})).into_response()
            }
            Self::Unauthorized => {
                return (
                    self.status_code(),
                    [(WWW_AUTHENTICATE, HeaderValue::from_static("Token"))]
                        .into_iter()
                        .collect::<HeaderMap>(),
                    self.to_string()
                ).into_response();
            }
            Self::Sqlx(ref e) => {
                log::error!("SQLx error: {:?}", e)
            }
            Self::Anyhow(ref e) => {
                log::error!("Generic error: {:?}", e)
            }
            _ => {}
        }

        (self.status_code(), self.to_string()).into_response()
    }
}

pub trait ResultExt<T> {
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error>;
}

impl <T, E> ResultExt<T> for Result<T, E>
where
    E: Into<Error> {
    
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T, Error> {
        self.map_err(|err| match err.into() {
            Error::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e
        })
    }
}