use std::process::Command;
use std::str::from_utf8;

pub async fn check(files: Vec<String>) -> Vec<String> {
    let mut diffs: Vec<String> = vec![];

    for file in files {
        let output = Command::new("statix")
            .arg("fix")
            .arg("-d")
            .arg(file)
            .output()
            .expect("Failed to execute statix").stdout;
        diffs.push(format!("{}", from_utf8(&output).unwrap()))
    }
    diffs
}
