{
  description = "A flake for building the server that serves the website";

  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.05";

    crane.url = "github:ipetkov/crane";

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, crane, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        inherit (pkgs) lib;

        craneLib = (crane.mkLib pkgs).overrideToolchain
          (p: p.rust-bin.selectLatestNightlyWith
            (toolchain: toolchain.default)
          );

        website = craneLib.buildPackage
          {
            pname = "website";
            version = "0.1";
            cargoExtraArgs = "-F production";
            strictDeps = true;
            doCheck = false;
            src = lib.fileset.toSource {
              root = ./.;
              fileset = lib.fileset.unions [
                ./Cargo.toml
                ./Cargo.lock
                ./src
								./public
              ];
            };
            postInstall = "cp -r public $out/public";
          };
      in
      {
        packages = {
          default = website;
        };
      });
}
