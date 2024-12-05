use std::process::{Command, Output};
use std::io::{self, Write};
use std::os::windows::process::ExitStatusExt;

pub fn load() {
    println!("Loading Sync module...");
    main();
}

pub(crate) fn run_syncall(dc1_ip: &str, dc2_ip: &str) -> Result<Output, String> {
    println!("Starting AD synchronization between {} and {}...", dc1_ip, dc2_ip);

    // commands for both domain controllers
    let commands = vec![
        format!("repadmin /syncall {} /e /d /A /P", dc1_ip),
        format!("repadmin /syncall {} /e /d /A /P", dc2_ip),
    ];

    for command in commands {
        // exec
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

fn main() {
    let dc1_ip = "192.168.1.1"; // Replace with your DC1 IP address
    let dc2_ip = "192.168.1.2"; // Replace with your DC2 IP address

    match run_syncall(dc1_ip, dc2_ip) {
        Ok(output) => {
            println!("AD synchronization successfully completed!");
            io::stdout().write_all(&output.stdout).unwrap();
        }
        Err(error) => {
            eprintln!("Error during AD synchronization: {}", error);
        }
    }
}