import json
import subprocess
import tempfile
import click

import checks.build 

@click.command()
@click.argument('pr_num', type=click.UNPROCESSED)
def main(pr_num):
    with tempfile.TemporaryDirectory(
        prefix=f"warden-pr-{pr_num}-", dir="/tmp"
    ) as temp_dir:
        # print(f"\n📂 Created temporary workspace at: {temp_dir}")
        # branch_name = json.loads(subprocess.run(
        #     f"gh pr view {pr_num} -R nixos/nixpkgs --json headRefName",
        #     shell=True, capture_output=True, text=True
        # ).stdout)["headRefName"]
        #
        # print(f"📥 Cloning branch {branch_name}...")
        # subprocess.run(
        #     f"git clone --filter=tree:0 --single-branch --depth=1000 -n https://github.com/nixos/nixpkgs temp_dir",
        #     shell=True, cwd=temp_dir
        # )
        # subprocess.run(
        #     f"gh pr checkout {pr_num} -R nixos/nixpkgs",
        #     shell=True, cwd=temp_dir
        # )

        print(f"📋 Performing checks...")
        checks.build.run(pr_num)

