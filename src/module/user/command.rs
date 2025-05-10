// module/user/command.rs
use crate::module::user::dto::{RegisterDto, LoginDto};
use crate::module::user::model::User;
use crate::core::error::AppError;
use sqlx::{PgExecutor, query_as};
use uuid::Uuid;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use password_hash::{SaltString, PasswordHash as ParsedHash};
use rand_core::OsRng;

pub async fn register_user(
    executor: impl PgExecutor<'_>,
    dto: RegisterDto,
) -> Result<User, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Argon2::default()
        .hash_password(dto.password.as_bytes(), &salt)
        .map_err(|e| AppError::internal(e.to_string()))?
        .to_string();

    let user = query_as::<_, User>(
        "INSERT INTO users (id, tenant_id, email, password_hash, is_active) \
         VALUES ($1, $2, $3, $4, true) RETURNING *"
    )
    .bind(Uuid::new_v4())
    .bind(dto.tenant_id)
    .bind(dto.email)
    .bind(password_hash)
    .fetch_one(executor)
    .await?;

    Ok(user)
}

pub async fn verify_user(
    executor: impl PgExecutor<'_>,
    dto: LoginDto,
) -> Result<User, AppError> {
    let user = query_as::<_, User>(
        "SELECT * FROM users WHERE email = $1 AND is_active = true"
    )
    .bind(&dto.email)
    .fetch_optional(executor)
    .await?;

    let user = user.ok_or_else(|| AppError::unauthorized("Invalid credentials"))?;

    let parsed_hash = ParsedHash::new(&user.password_hash)
        .map_err(|_| AppError::unauthorized("Invalid password"))?;

    let is_valid = Argon2::default()
        .verify_password(dto.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !is_valid {
        return Err(AppError::unauthorized("Invalid password"));
    }

    Ok(user)
}