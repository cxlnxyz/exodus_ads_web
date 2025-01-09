use std::os::windows::process::ExitStatusExt;
use std::process::{Command, Output};

pub fn run_syncall(dc1_ip: &str, dc2_ip: &str) -> Result<Output, String> {
    println!("Starting AD synchronization between {} and {}...", dc1_ip, dc2_ip);

    let commands = vec![
        format!("repadmin /syncall {} /e /d /A /P", dc1_ip),
        format!("repadmin /syncall {} /e /d /A /P", dc2_ip),
    ];

    for command in commands {
        let output = Command::new("cmd")
            .args(&["/C", &command])
            .output()
            .map_err(|e| format!("Error starting command: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }
    }

    Ok(Output {
        status: std::process::ExitStatus::from_raw(0),
        stdout: Vec::new(),
        stderr: Vec::new(),
    })
}