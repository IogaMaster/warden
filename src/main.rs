mod checks;

use std::{borrow::Borrow, collections::HashMap, env, error::Error, fmt::format, process::{exit, Command}};

use mktemp::Temp;
use octocrab::models::pulls::FileDiff;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut pr_num: String = String::new();

    if args.len() > 1 {
        if args[1].contains("github.com") == true {
            pr_num = args[1].split("/").last().unwrap().to_string();
        } else if args[1].to_string().parse::<i32>().is_ok() {
            pr_num = args[1].to_string();
        };
    } else {
        println!("You need to specify the pr number or url");
        exit(1);
    }

    let mut changed_packages: Vec<String> = vec![];
    
    let github =  octocrab::instance();
    let nixpkgs = github.pulls("nixos", "nixpkgs");

    for diff in nixpkgs.list_files(pr_num.parse::<u64>().unwrap())
        .await.unwrap()
    {
        if diff.filename.contains("pkgs") && diff.filename.contains("by-name") {
            changed_packages.push(diff.filename);
        }
    };


    let nixpkgs_source = Temp::new_dir().unwrap();
    
    let pr_branch = nixpkgs.get(pr_num.parse::<u64>().unwrap()).await.unwrap().head.ref_field;
    let pr_repo = nixpkgs.get(pr_num.parse::<u64>().unwrap()).await.unwrap().head.repo.unwrap().html_url.unwrap();
    let pr_commits_num = nixpkgs.get(pr_num.parse::<u64>().unwrap()).await.unwrap().commits.unwrap();

    let output = Command::new("git")
        // Clone the forked nixpkgs repo
        .arg("clone")
        .arg(format!("https://github.com{}", pr_repo.path()))

        // Clone the branch from the pr
        .arg("-b")
        .arg(pr_branch)
        // Filter blobs to make the clone faster
        .arg("--filter=tree:0")
        // Clone the repo to a depth of 10 
        .arg("--depth=10")
        // Clone to the tmp dir
        .arg(nixpkgs_source.as_ref().as_os_str())
        .output()
        .expect("Failed to clone nixpkgs fork!!!");

   let statix_diffs = checks::statix::create_report(&changed_packages, &nixpkgs_source).await;
   let deadnix_diffs = checks::deadnix::check(&changed_packages, &nixpkgs_source).await;
   let nixpkgs_hammering_logs = checks::hammering::check(&nixpkgs_source, &pr_commits_num).await;
   let nix_build_logs = checks::build::check(&nixpkgs_source, &pr_commits_num).await;
   
   let report = create_report(statix_diffs, deadnix_diffs, nixpkgs_hammering_logs, nix_build_logs);
   println!("{report}")
}

fn create_report(statix_diffs: Option<String>, deadnix_diffs: Option<String>, nixpkgs_hammering_logs: Option<String>, nix_build_logs: Option<String>) -> String {

    let mut package_report = String::new();
    let mut basic_lint_report = String::new();
    let mut advanced_lint_report = String::new();

    let nix_build_report = match nix_build_logs {
        Some(s) => s,
        None => "".to_string(),
    };

    if nix_build_report != "" {
        package_report = format!(r"
<details><summary>Packages built</summary>
<p>

{}

</p>
</details>", nix_build_report);
    }

    let statix_report = match statix_diffs {
        Some(s) => s,
        None => "".to_string(),
    };

    let deadnix_report = match deadnix_diffs {
        Some(s) => s,
        None => "".to_string(),
    };

    if statix_report != "" && deadnix_report != "" {
        basic_lint_report = format!(r"
<details><summary>Basic Lints</summary>
<p>

{}
{}

</p>
</details>", statix_report, deadnix_report);
    }


    let nixpkgs_hammering_report = match nixpkgs_hammering_logs {
        Some(s) => s,
        None => "".to_string(),
    };

    if nixpkgs_hammering_report != "" {
        advanced_lint_report = format!(r"
<details><summary>Special Lints</summary>
<p>

{}

</p>
</details>", nixpkgs_hammering_report);
    }

    String::from(format!(r"
This review has been done with [warden](https://github.com/iogamaster/warden), please report any issues!

{} 
{}
{}
", package_report, basic_lint_report, advanced_lint_report))
}
