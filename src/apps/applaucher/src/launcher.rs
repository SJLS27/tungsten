use std::process::Command;

// Logic for launching the selected application

pub fn launch_app(exec: &str) {
    let mut parts = exec.split_whitespace();
    if let Some(cmd) = parts.next() {
        let args: Vec<&str> = parts.collect();
        let _ = Command::new(cmd)
            .args(&args)
            .spawn();
    }
}