use askama::Template;
use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_csrf::CsrfToken;
use html_minifier::minify;

use crate::state::AppState;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPageTemplate {
    csrf_token: String,
}

pub async fn serve_index(State(_state): State<AppState>, token: CsrfToken) -> impl IntoResponse {
    let csrf_token = token.authenticity_token().unwrap();

    let template = IndexPageTemplate { csrf_token };
    let rendered = template.render().unwrap();
    let minified = minify(&rendered).unwrap();

    (token, Html(minified)).into_response()
}

#[derive(Template)]
#[template(path = "apply.html")]
struct ApplyPageTemplate {
    csrf_token: String,
    recaptcha_site_token: String,
}

pub async fn serve_apply_form(
    State(_state): State<AppState>,
    token: CsrfToken,
) -> impl IntoResponse {
    let csrf_token = token.authenticity_token().unwrap();
    let recaptcha_site_token = env!("RECAPTCHA_SITE_KEY").to_string();

    let template = ApplyPageTemplate {
        csrf_token,
        recaptcha_site_token,
    };
    let rendered = template.render().unwrap();
    let minified = minify(&rendered).unwrap();

    (token, Html(minified)).into_response()
}

#[derive(Template)]
#[template(path = "about.html")]
struct AboutPageTemplate;

pub async fn serve_about_page(State(_state): State<AppState>) -> impl IntoResponse {
    let template = AboutPageTemplate;
    let rendered = template.render().unwrap();
    let minified = minify(&rendered).unwrap();

    Html(minified).into_response()
}
