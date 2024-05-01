{
  lib,
  stdenv,
  darwin,
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
  ] ++ lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
  ];

  meta = with lib; {
    description = "A self hostable nixpkgs review bot";
    homepage = "https://github.com/IogaMaster/warden";
    license = licenses.mit;
    maintainers = with maintainers; [ iogamaster ];
    mainProgram = "warden";
  };
}
