use std::process::{Command, Output};

pub fn run_powershell_script(username: &str, password: &str) -> Result<Output, String> {
    let script_path = "src/loader/server/check_ad_login.ps1";

    let output = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(script_path)
        .arg(username)
        .arg(password)
        .output()
        .map_err(|e| format!("Failed to execute PowerShell script: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(output)
}


pub fn get_ad_users() -> Result<Output, String> {
    let script_path = "src/loader/server/get_ad_users.ps1";

    let output = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass")
        .arg("-File")
        .arg(script_path)
        .output()
        .map_err(|e| format!("Failed to execute PowerShell script: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(output)
}