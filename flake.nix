{
  description = "A self hostable nixpkgs update bot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = nixpkgs.legacyPackages.${system};
      in rec {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rust-analyzer
          ];

          inputsFrom = [ packages.warden ];
        };

        packages = {
          warden = pkgs.callPackage ./nix/package.nix {
            version = self.shortRev or self.dirtyShortRev or "unknown";
          };

          container = pkgs.callPackage ./nix/docker.nix {
            inherit (packages) warden;
          };

          default = packages.warden;
        };
      }
    );
}
