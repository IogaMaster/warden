{
  description = "A self hostable nixpkgs update bot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    naersk.url = "github:nix-community/naersk";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    nixpkgs,
    rust-overlay,
    flake-utils,
    naersk,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        
        naersk' = pkgs.callPackage naersk {};

        nativeBuildInputs = with pkgs; [
          rust-bin.stable.latest.default
          rust-analyzer
        ];

        buildInputs = with pkgs; [
            nixpkgs-review
            nixpkgs-hammering
            statix
            deadnix
            eza
        ];
      in {
        devShells.default = pkgs.mkShell {inherit nativeBuildInputs buildInputs;};

        packages.default = naersk'.buildPackage {
            src = ./.;
            inherit nativeBuildInputs buildInputs;
        };
      }
    );
}
