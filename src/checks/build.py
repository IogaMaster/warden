import logging
import subprocess
import json


def run(pr_num):
    logging.debug("Getting commits...")
    commits_ret = json.loads(subprocess.run(
        f"gh pr view {pr_num} -R nixos/nixpkgs --json commits",
        shell=True, capture_output=True, text=True
    ).stdout)["commits"]

    commits = []
    for commit in commits_ret:
        commits.append(commit["messageHeadline"])
