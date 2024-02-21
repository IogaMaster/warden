use std::path::Path;
use std::process::Command;
use std::str::from_utf8;

pub async fn check(files: Vec<String>, nixpkgs_source: &Path) -> Vec<String> {
    let mut diffs: Vec<String> = vec![];

    for file in files {
        let output = Command::new("statix")
            .current_dir(&nixpkgs_source.as_os_str())
            .arg("fix")
            .arg("-d")
            .arg(file)
            .output()
            .expect("Failed to execute statix");
        diffs.push(format!("{}", from_utf8(&output.stdout).unwrap()))
    } 
    diffs
}
