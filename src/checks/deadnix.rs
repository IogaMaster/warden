use std::path::Path;
use std::process::Command;
use std::str::from_utf8;

pub async fn check(files: &Vec<String>, nixpkgs_source: &Path) -> Vec<String> {
    let mut diffs: Vec<String> = vec![];

    for file in files {
        let output = Command::new("deadnix") 
            .current_dir(&nixpkgs_source.as_os_str())
            .arg("-e")
            .arg(file)
            .output()
            .expect("Failed to execute deadnix").stdout;

        let git_diff = Command::new("git")
            .current_dir(&nixpkgs_source.as_os_str())
            .arg("diff")
            .arg("HEAD")
            .output()
            .expect("Failed to execute git").stdout;
        diffs.push(format!("{}", from_utf8(&git_diff).unwrap()));
        let git_reset = Command::new("git")
            .current_dir(&nixpkgs_source.as_os_str())
            .arg("reset")
            .arg("HEAD")
            .arg("--hard")
            .output()
            .expect("Failed to execute git").stdout;
    }
    diffs
}
