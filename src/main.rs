mod checks;

use std::{collections::HashMap, env, error::Error, process::{exit, Command}, borrow::Borrow};

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

    println!("{:#?}", checks::statix::check(changed_packages, &nixpkgs_source).await);
}
