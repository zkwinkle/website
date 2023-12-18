{ pkgs }:
pkgs.rustPlatform.buildRustPackage rec {
	pname = "website";
	version = "0.1";
	cargoLock.lockFile = ./Cargo.lock;
	src = pkgs.lib.cleanSource ./.;
}
