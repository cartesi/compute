use std::process::{Command, Output};

fn encode_sig(sig: &str) -> String {
    let cmd = format!("cast sig-event \"{}\"", sig);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("Failed to execute command");

    let encoded_sig = String::from_utf8_lossy(&output.stdout).to_string();
    encoded_sig
}

fn decode_event_data(sig: &str, data: &str) -> Vec<String> {
    let cmd = format!("cast --abi-decode \"bananas()%s\" {} {}", sig, data);
    let output = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let ret: Vec<String> = stdout.lines().map(String::from).collect();
    ret
}