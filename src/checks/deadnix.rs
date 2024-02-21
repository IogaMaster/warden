use std::process::Command;
use std::str::from_utf8;

pub async fn check(files: Vec<String>) -> Vec<String> {
    let mut diffs: Vec<String> = vec![];

    for file in files {
        let output = Command::new("deadnix")
            .arg("-e")
            .arg(file)
            .output()
            .expect("Failed to execute deadnix").stdout;
        diffs.push(format!("{}", from_utf8(&output).unwrap()))
    }
    diffs
}
