{
  lib,
  rustPlatform,
  nixpkgs-review,
  nixpkgs-hammering,
  statix,
  deadnix,
  version ? (lib.importTOML ../Cargo.toml).package.version,
}:
rustPlatform.buildRustPackage {
  pname = "warden";
  inherit version;

  src = lib.fileset.toSource {
    root = ../.;
    fileset = lib.fileset.unions [
      ../src
      ../Cargo.toml
      ../Cargo.lock
    ];
  };

  cargoLock = {
    lockFile = ../Cargo.lock;
    allowBuiltinFetchGit = true;
  };

  buildInputs = [
    nixpkgs-review
    nixpkgs-hammering
    statix
    deadnix
  ];
}
