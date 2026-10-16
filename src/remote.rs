use std::{
    io::Write,
    process::{Command, ExitStatus, Stdio},
};

pub fn run_script(host: &str, script: &str) -> std::io::Result<ExitStatus> {
    let mut child = Command::new("ssh")
        .args(["-o", "BatchMode=yes"])
        .arg(host)
        .args(["sh", "-s"])
        .stdin(Stdio::piped())
        .spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(script.as_bytes())?;
    }

    child.wait()
}

pub fn check_host(host: &str) -> std::io::Result<ExitStatus> {
    Command::new("ssh")
        .args([
            "-o",
            "BatchMode=yes",
            "-o",
            "ConnectTimeout=3",
            host,
            "true",
        ])
        .status()
}
