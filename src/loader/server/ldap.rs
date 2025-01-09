use std::process::{Command, Output};
use std::io::{self, Write};
use actix_web::{web, HttpResponse, Error};
use serde::{Deserialize, Serialize};

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
    match run_powershell_script(&login.username, &login.password) {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            if output_str.contains("Login successful") {
                Ok(HttpResponse::Ok().body("Login successful"))
            } else {
                Ok(HttpResponse::Unauthorized().body("Invalid credentials"))
            }
        }
        Err(e) => {
            eprintln!("Error executing PowerShell script: {}", e);
            Ok(HttpResponse::InternalServerError().body("Authentication error"))
        }
    }
}

pub(crate) fn run_powershell_script(username: &str, password: &str) -> Result<Output, String> {
    let script_path = "src/loader/server/ldap.ps1"; // Path to your PowerShell script

    let output = Command::new("powershell")
        .args(&["-File", script_path, username, password])
        .output()
        .map_err(|e| format!("Failed to execute PowerShell script: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(output)
}