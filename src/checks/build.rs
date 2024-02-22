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

    let mut build_logs: Vec<String> = vec![];
    
    for pkg in packages {
        match Command::new("nix")
            .current_dir(&nixpkgs_source.as_os_str())
            .env("NIXPKGS_ALLOW_UNFREE", "1")
            .env("NIXPKGS_ALLOW_BROKEN", "1")
            .env("NIXPKGS_ALLOW_UNSUPPORTED_SYSTEM", "1")
            .arg("build")
            .arg("--rebuild")
            .arg(format!(".#{}", &pkg))
            .output() {
                Ok(_) => build_logs.push(format!(":heavy_check_mark: {}", pkg)),
                Err(_) => build_logs.push(format!(":x: {}", pkg)),  
            };
    }

    if build_logs.last().is_some() {
        Some(format!("{}", build_logs.join("\n\n")))
    } else {
        None
    }
}

