use axum::{extract::{Json, Extension}, response::IntoResponse};
use axum_macros::debug_handler;
use crate::module::user::dto::{LoginDto, LoginResponse, RegisterDto};
use crate::infra::db::PgConn;
use crate::core::error::AppError;
use crate::module::user::command::*;

#[debug_handler]
pub async fn register(
    Extension(conn): Extension<PgConn>,
    Json(payload): Json<RegisterDto>,
) -> Result<impl IntoResponse, AppError> {
    let user = register_user(&conn, payload).await?;
    let token = crate::core::jwt::encode_user(&user)?;
    Ok(Json(LoginResponse { token }))
}

#[debug_handler]
pub async fn login(
    Extension(conn): Extension<PgConn>,
    Json(payload): Json<LoginDto>,
) -> Result<impl IntoResponse, AppError> {
    let user = verify_user(&conn, payload).await?;
    let token = crate::core::jwt::encode_user(&user)?;
    Ok(Json(LoginResponse { token }))
}