use std::process::{Command, Output};
use std::io::{self, Write};
use actix_web::{HttpResponse, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub(crate) username: String,
    pub(crate) password: String,
}

pub fn run_powershell_script(username: &str, password: &str) -> Result<Output, String> {
    let script_path = "src/loader/server/ldap.ps1"; // Path to your PowerShell script

    let output = Command::new("powershell")
        .args(&["-ExecutionPolicy", "Bypass", "-File", script_path, username, password])
        .output()
        .map_err(|e| format!("Failed to execute PowerShell script: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(output)
}