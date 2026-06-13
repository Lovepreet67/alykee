use axum::{Extension, Json, extract::State, http::StatusCode};

use crate::{
    error::APIError,
    model::{
        ListedResponse,
        auth::{
            JWTResponse, LoginRequest,
            LoginResponse::{self},
            TokenVerificationRequest, TwoFAResponse,
        },
        tasks::Task,
    },
    repository::{tasks::TaskRepository, users::UserRepository},
    service::{
        self,
        auth::{Claims, sign_jwt},
        email::send_email,
        redis::otp::{del_otp, have_active_otp, set_otp},
    },
    state::AppState,
    utilities::{self, generate_otp_message},
};

// this should be paginated
pub async fn get_tasks(
    State(app_state): State<AppState>,
    Extension(auth_claims): Extension<Claims>,
) -> Result<Json<ListedResponse<Task>>, APIError> {
    let pool = app_state.pool;
    let user_id = auth_claims.sub;
    let tasks = TaskRepository::list_by_user_id(&pool, user_id).await?;
    let listed = ListedResponse { list: tasks };
    Ok(Json(listed))
}

pub async fn login(
    State(app_state): State<AppState>,
    Json(login_req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, APIError> {
    let pool = app_state.pool;
    let user = UserRepository::find_by_email(&pool, &login_req.email).await?;
    if user.is_none() {
        return Err(APIError::new(
            StatusCode::NOT_FOUND,
            "Incorrect Username or passwor",
            "INVALID_CREDENTIALS",
        ));
    }
    let user = user.unwrap();
    let test = bcrypt::verify(&login_req.password, &user.hashed_password)?;
    if !test {
        return Err(APIError::new(
            StatusCode::NOT_FOUND,
            "Incorrect Username or passwor",
            "INVALID_CREDENTIALS",
        ));
    }
    // as all users are 2FA enabled we will generate the otp
    // check if the otp already exists,
    let active_otp = have_active_otp(&app_state.redis_client, user.id).await?;
    if active_otp {
        return Err(APIError::new(
            StatusCode::NOT_ACCEPTABLE,
            "OTP Already shared please wait it to expire to get new otp",
            "ALREADY_SHARED",
        ));
    }
    // other wise we will generate the otp
    let otp = utilities::get_otp(Some(10));
    // we will use the challenge_id
    set_otp(&app_state.redis_client, user.id, &otp).await?;
    // send the email
    let message = generate_otp_message(&user.full_name, &user.email, &otp)?;
    send_email(message)?;
    set_otp(&app_state.redis_client, user.id, &otp).await?;
    // else we will generate the jwt
    Ok(Json(LoginResponse::TwoFA(TwoFAResponse {
        message: "OTP Shared on registed email".into(),
    })))
}

pub async fn verify_otp(
    State(app_state): State<AppState>,
    Json(token_req): Json<TokenVerificationRequest>,
) -> Result<Json<LoginResponse>, APIError> {
    let user = match UserRepository::find_by_email(&app_state.pool, &token_req.email).await? {
        Some(user) => user,
        None => {
            return Err(APIError::new(
                StatusCode::NOT_FOUND,
                "Email not registered please consider signing in",
                "INVALID_EMAIL",
            ));
        }
    };
    // check if the otp already exists,
    let otp = match service::redis::otp::get_otp(&app_state.redis_client, user.id).await? {
        Some(v) => v,
        None => {
            return Err(APIError::new(
                StatusCode::NOT_FOUND,
                "No Active OTP Found",
                "OTP_NOT_FOUND",
            ));
        }
    };
    if otp != token_req.otp {
        return Err(APIError::new(
            StatusCode::BAD_REQUEST,
            "Shared otp is not valid",
            "INVALID_OTP",
        ));
    }
    let jwt = sign_jwt(&user).await?;
    del_otp(&app_state.redis_client, user.id).await?;
    Ok(Json(LoginResponse::Jwt(JWTResponse { jwt })))
}
