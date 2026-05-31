{
  lib,
  nix,
  nixpkgs-review,
  nixpkgs-lint-community,
  nixpkgs-hammering,
  statix,
  deadnix,
  gh,
  python3Packages,
  version ? "latest",
}:
python3Packages.buildPythonApplication {
  pname = "warden";
  inherit version;
  pyproject = true;

  src = ../.;

  build-system = [ python3Packages.setuptools ];

  propagatedBuildInputs = [
    python3Packages.click
  ];

  makeWrapperArgs = [
    "--prefix"
    "PATH"
    ":"
    (lib.makeBinPath [
      nix
      nixpkgs-review
      nixpkgs-lint-community
      nixpkgs-hammering
      statix
      deadnix
      gh
    ])
  ];

}
