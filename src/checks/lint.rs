use std::path::Path;
use std::process::Command;
use std::str::from_utf8;

pub async fn check(nixpkgs_source: &Path, num_commits: &u64) -> Option<String> {
    let mut packages: Vec<String> = vec![];

    let git_log = Command::new("git")
        .current_dir(&nixpkgs_source.as_os_str())
        .arg("log")
        .arg(format!("-{}", num_commits))
        // Print only the commit title
        .arg("--pretty=format:%s")
        .output()
        .expect("Failed to execute git");


    for commit in from_utf8(&git_log.stdout).unwrap().lines() {
        if commit.contains("init") || commit.contains("->") {
            let pkg_name = &commit[..commit.find(':').unwrap()];
            packages.push(pkg_name.to_string());
        }
    }
    
    let mut lint_logs: Vec<String> = vec![];
    
    for pkg in packages {
        let output = Command::new("nixpkgs-lint")
            .current_dir(&nixpkgs_source.as_os_str())
            .env("NIXPKGS_ALLOW_UNFREE", "1")
            .env("NIXPKGS_ALLOW_BROKEN", "1")
            .env("NIXPKGS_ALLOW_UNSUPPORTED_SYSTEM", "1")
            .arg("-p")
            .arg(&pkg)
            .output()
            .expect("Failed to execute nixpkgs-lint");

        let diff_string = from_utf8(&output.stdout).unwrap();
        if diff_string == "" {
            continue;
        }
        lint_logs.push(format!("{}", diff_string))
    }
    if lint_logs.last().is_some() {
        Some(format!("## `nixpkgs-lint` report: \n\n{}", lint_logs.join("\n")))
    } else {
        None
    }
}

