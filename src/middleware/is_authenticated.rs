use axum::{Extension, extract::Request, http::StatusCode, middleware::Next, response::Response};

use crate::{
    error::APIError,
    model::users::UserRole,
    service::auth::{Claims, verify_jwt},
};
const JWT_HEADER: &'static str = "Auth";
pub async fn is_authenticated(mut req: Request, next: Next) -> Result<Response, APIError> {
    let header_val = req.headers().get(JWT_HEADER).map(|x| x.to_str());
    if (header_val.is_none()) {
        return Err(APIError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid Jwt token",
            "INVALID_TOKEN",
        ));
    };
    //todo: remove this unwraps
    let claims = verify_jwt(header_val.unwrap().unwrap())?;
    req.extensions_mut().insert(claims);
    Ok(next.run(req).await)
}

pub async fn have_admin_role(req: Request, next: Next) -> Result<Response, APIError> {
    let is_admin = req
        .extensions()
        .get::<Claims>()
        .map(|c| c.role == UserRole::Admin)
        .unwrap_or(false);

    if !is_admin {
        return Err(APIError::new(
            StatusCode::UNAUTHORIZED,
            "This resource is only accessible to admin",
            "ADMIN_ACCESS_ONLY",
        ));
    }

    Ok(next.run(req).await)
}
