use std::process::Command;
use std::collections::HashMap;

pub fn get_jobs() -> HashMap<String, String> {
    let output = Command::new("squeue")
        .args(["-u", &std::env::var("USER").unwrap(),
               "-h", "-o", "%i %t"])
        .output()
        .expect("failed to run squeue");

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut jobs = HashMap::new();

    for line in stdout.lines() {
        let mut parts = line.split_whitespace();
        let id = parts.next().unwrap().to_string();
        let state = parts.next().unwrap().to_string();
        jobs.insert(id, state);
    }

    jobs
}
