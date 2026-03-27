use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: msbatch <script.sh>");
        return;
    }

    let script = &args[1];

    let output = Command::new("sbatch")
        .arg(script)
        .output()
        .expect("failed to run sbatch");

    let stdout = String::from_utf8_lossy(&output.stdout);

    println!("{}", stdout);

    let job_id = stdout
        .split_whitespace()
        .last()
        .expect("could not parse job id");

    let name = script.replace(".sh", "").replace("./", "");

    // Append to state file
    let mut state = cluster_monitor::state::load();

    state.runs.push(cluster_monitor::state::Run {
        name,
        job_id: job_id.to_string(),
        status: cluster_monitor::state::Status::Pending,
        checked: false,
    });

    cluster_monitor::state::save(&state);
}
