use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    success: bool,
}

pub(crate) async fn ldap_login(login: web::Json<LoginRequest>) -> Result<HttpResponse, Error> {
    let api_url = "http://127.0.0.1:8080/login";
    let client = Client::new();

    let response = client.post(api_url)
        .json(&*login)
        .send()
        .await
        .map_err(|e| {
            eprintln!("Failed to send request to API: {:?}", e);
            actix_web::error::ErrorInternalServerError("Failed to send request to API")
        })?;

    if response.status().is_success() {
        let auth_response: AuthResponse = response.json().await.map_err(|e| {
            eprintln!("Failed to parse API response: {:?}", e);
            actix_web::error::ErrorInternalServerError("Failed to parse API response")
        })?;

        if auth_response.success {
            Ok(HttpResponse::Ok().body("Login successful"))
        } else {
            Ok(HttpResponse::Unauthorized().body("Invalid credentials"))
        }
    } else {
        Ok(HttpResponse::InternalServerError().body("API request failed"))
    }
}