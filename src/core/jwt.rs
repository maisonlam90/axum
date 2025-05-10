use crate::module::user::model::User;
use crate::core::error::AppError;

pub fn encode_user(user: &User) -> Result<String, AppError> {
    // TODO: Replace with real JWT
    Ok(format!("token-for-{}", user.email))
}
