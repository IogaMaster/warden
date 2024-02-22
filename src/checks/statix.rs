use std::path::Path;
use std::process::Command;
use std::str::from_utf8;

pub async fn create_report(files: &Vec<String>, nixpkgs_source: &Path) -> Option<String> {
    let mut diffs: Vec<String> = vec![];

    for file in files {
        let output = Command::new("statix")
            .current_dir(&nixpkgs_source.as_os_str())
            .arg("fix")
            .arg("-d")
            .arg(file)
            .output()
            .expect("Failed to execute statix");
        let diff_string = from_utf8(&output.stdout).unwrap();
        if diff_string == "" {
            continue;
        }
        diffs.push(diff_string.to_string());
    } 

    if diffs.last().is_some() {
        Some(format!("\n## Statix diffs:\n```diff\n{}```", diffs.join("```\n ```diff\n")))
    } else {
        None
    }
}
