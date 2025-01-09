use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};
use crate::loader::server::ldap::run_powershell_script;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct AuthResponse {
    success: bool,
}

pub(crate) async fn ldap_login(login: web::Json<LoginRequest>) -> Result<HttpResponse, Error> {
    println!("Running PowerShell script for username: {}", login.username);
    match run_powershell_script(&login.username, &login.password) {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            println!("PowerShell script output: {}", output_str);
            if output_str.contains("Login successful") {
                Ok(HttpResponse::Ok().body("Login successful"))
            } else {
                println!("Invalid credentials: {}", output_str);
                Ok(HttpResponse::Unauthorized().body("Invalid credentials"))
            }
        }
        Err(e) => {
            eprintln!("Error executing PowerShell script: {}", e);
            Ok(HttpResponse::InternalServerError().body("Authentication error"))
        }
    }
}