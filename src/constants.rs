// Enviame - Full-stack Priority Messenger with a Rust backend that respects priority settings and delivers messages.
// Copyright (C) 2025 Brian Chen (differental)
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use chrono_tz::Tz;
use lettre::{
    SmtpTransport,
    transport::smtp::authentication::{Credentials, Mechanism},
};
use std::{env, sync::LazyLock};

// --- Calendar ---
// Your timezone, used as the timezone of "00:00" for all-day events and the timezone for sleep-time blocking periods
pub static DEFAULT_TZ: LazyLock<Tz> = LazyLock::new(|| {
    env::var("LOCAL_TIMEZONE")
        .unwrap_or("UTC".to_owned())
        .trim()
        .parse::<Tz>()
        .unwrap()
});

// --- Formats ---
// Datetime format, used when sending emails
pub const EMAIL_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

// Datetime format, used in calendar API. Frontend js must be updated if this is changed
pub const CALENDAR_DATETIME_FORMAT: &str = "%Y-%m-%d %H:%M";

// --- General Configuration ---
// Homepage URL, relevant when sending login links to users
pub static HOMEPAGE_URL: LazyLock<String> =
    LazyLock::new(|| env::var("HOMEPAGE_URL").expect("HOMEPAGE_URL must be set"));

// Deploy environment, relevant in displaying beta warning and modifying db below
pub static DEPLOY_ENV: LazyLock<String> =
    LazyLock::new(|| env::var("DEPLOY_ENV").expect("DEPLOY_ENV must be set"));

// Whether to allow registration and form submission requests
pub static ALLOW_MODIFY_DB: LazyLock<bool> =
    LazyLock::new(|| *DEPLOY_ENV == "prod" || *DEPLOY_ENV == "beta");

// Cargo package version, as specified in Cargo.toml
pub const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

// --- Keys and Secrets ---
// Message ID Hash Key, used in message query API
pub static MID_HASH_KEY: LazyLock<String> =
    LazyLock::new(|| env::var("HASH_KEY").expect("HASH_KEY must be set"));

// Recaptcha site key, embedded in HTML
pub static RECAPTCHA_SITE_KEY: LazyLock<String> =
    LazyLock::new(|| env::var("RECAPTCHA_SITE_KEY").expect("RECAPTCHA_SITE_KEY must be set"));

// Recaptcha secret key, used when verifying requests
pub static RECAPTCHA_SECRET_KEY: LazyLock<String> =
    LazyLock::new(|| env::var("RECAPTCHA_SECRET_KEY").expect("RECAPTCHA_SECRET_KEY must be set"));

// --- SMTP Email Configurations ---
// Recipient address of all notification emails, and reply_to address of all user emails
pub static NOTIFICATION_EMAIL: LazyLock<String> =
    LazyLock::new(|| env::var("NOTIFICATION_EMAIL").expect("NOTIFICATION_EMAIL must be set"));

// SMTP Server
pub static SMTP_SERVER: LazyLock<String> =
    LazyLock::new(|| env::var("SMTP_SERVER").expect("SMTP_SERVER must be set"));

// SMTP Credentials
pub static SMTP_CREDS: LazyLock<Credentials> = LazyLock::new(|| {
    let smtp_username = env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set");
    let smtp_password = env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set");
    Credentials::new(smtp_username, smtp_password)
});

// SMTP Port
pub static SMTP_PORT: LazyLock<u16> = LazyLock::new(|| {
    env::var("SMTP_PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(587)
});

// SMTP Mailer using lettre
pub static MAILER: LazyLock<SmtpTransport> = LazyLock::new(|| {
    SmtpTransport::starttls_relay(SMTP_SERVER.as_str())
        .expect("Invalid SMTP server")
        .port(*SMTP_PORT)
        .credentials(SMTP_CREDS.clone())
        .authentication(vec![Mechanism::Plain])
        .build()
});

// SMTP FROMs, can be different from SMTP_USERNAME
pub static FROM_STANDARD: LazyLock<String> =
    LazyLock::new(|| env::var("SMTP_FROM").expect("SMTP_FROM must be set"));

pub static FROM_URGENT: LazyLock<String> =
    LazyLock::new(|| env::var("SMTP_FROM_URGENT").unwrap_or((*FROM_STANDARD).clone()));

pub static FROM_IMMEDIATE: LazyLock<String> =
    LazyLock::new(|| env::var("SMTP_FROM_IMMEDIATE").unwrap_or((*FROM_STANDARD).clone()));
