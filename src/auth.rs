use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::f32::consts::E;
use std::mem::transmute;

use actix_web_httpauth::extractors::AuthenticationError;
use alcoholic_jwt::{JWKS, token_kid, validate, Validation};
use serde::{Deserialize, Serialize};

use crate::auth;
use crate::errors::ServiceError;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}

// Basic authentication method (to be improved)
pub fn validate_user(username: &str, password: &str) -> Result<bool, ServiceError> {
    // Load the expected username and password from environment variables
    let expected_username = env::var("API_USER").expect("API_USER must be set");
    let expected_password = env::var("API_PASSWORD").expect("API_PASSWORD must be set");

    // Check if the provided username and password match the expected values
    if username == expected_username && password == expected_password {
        Ok(true)
    } else {
        Err((auth::ServiceError::BadRequest(password.parse().unwrap())))
    }
}