use axum::{
    Json,
    extract::State,
    http::{StatusCode, header},
    response::IntoResponse,
};
use axum_csrf::CsrfToken;
use reqwest::Client;
use serde::Deserialize;

use crate::constants::{ALLOW_MODIFY_DB, RECAPTCHA_SECRET_KEY};
use crate::state::AppState;
use crate::utils::generate_random_token;

#[derive(Deserialize)]
pub struct ApplyRequest {
    csrf_token: String,
    email: String,
    name: String,
    recaptcha: String,
}

#[derive(Deserialize)]
struct RecaptchaResponse {
    success: bool,
}

pub async fn handle_apply(
    State(state): State<AppState>,
    token: CsrfToken,
    Json(payload): Json<ApplyRequest>,
) -> impl IntoResponse {
    // Validate csrf token
    if token.verify(&payload.csrf_token).is_err() {
        return (StatusCode::BAD_REQUEST, "CSRF token invalid.").into_response();
    }

    // If not prod or beta, do not modify database. See constants
    if !*ALLOW_MODIFY_DB {
        return (
            StatusCode::IM_A_TEAPOT,
            "Account application ignored. This is not a production build.",
        )
            .into_response();
    }

    if payload.recaptcha.is_empty() {
        return (StatusCode::BAD_REQUEST, "reCAPTCHA verification failed").into_response();
    }

    let client = Client::new();
    let params = [
        ("secret", RECAPTCHA_SECRET_KEY.as_str()),
        ("response", &payload.recaptcha),
    ];

    let res = client
        .post("https://www.google.com/recaptcha/api/siteverify")
        .form(&params)
        .send()
        .await;

    match res {
        Ok(response) => {
            if let Ok(recaptcha_response) = response.json::<RecaptchaResponse>().await {
                if recaptcha_response.success {
                    let token = generate_random_token();

                    match sqlx::query!(
                        "INSERT INTO users (email, name, token, verified) VALUES ($1, $2, $3, $4)",
                        payload.email,
                        payload.name,
                        token,
                        false
                    )
                    .execute(&state.db)
                    .await
                    {
                        Ok(_) => (),
                        Err(_) => {
                            return (StatusCode::BAD_REQUEST, "Duplicate Email").into_response();
                        }
                    }

                    let cookie_header = format!("token={}; Path=/; Secure; SameSite=Strict", token);

                    return (
                        StatusCode::CREATED,
                        [(header::SET_COOKIE, cookie_header)],
                        "Registration submitted successfully!",
                    )
                        .into_response();
                }
            }
            (StatusCode::BAD_REQUEST, "reCAPTCHA verification failed").into_response()
        }
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error verifying reCAPTCHA",
        )
            .into_response(),
    }
}
