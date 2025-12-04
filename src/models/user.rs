use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// --- The Main User Entity ---
// Represents the combined data from 'users' AND 'profiles'
#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(FromRow)]
pub struct User {
    #[sqlx(try_from = "String")]
    pub uuid: String,

    pub email: String,

    #[sqlx(rename = "hash")]
    #[serde(skip_serializing)]
    pub password_hash: String,

    pub is_verified: bool,
    pub is_active: bool,
    pub token_version: i32,

    // --- Added from Profiles Table ---
    // These are Option<String> because your DB allows them to be NULL
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateUserSchema {
    pub email: String,
    pub password: String,
    // Frontend sends these during registration
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UserLoginSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub token_type: String,
}