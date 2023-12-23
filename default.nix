{ pkgs, makeWrapper }:
pkgs.rustPlatform.buildRustPackage rec {
  pname = "website";
  version = "0.1";
  cargoLock.lockFile = ./Cargo.lock;
  src = pkgs.lib.cleanSource ./.;
  nativeBuildInputs = [
    makeWrapper
  ];
  buildFeatures = [ "production" ];
  postInstall = ''
    		cp -r public $out/public
    		wrapProgram $out/bin/website \
    		--set PUBLIC_DIR "$out/public"
    		'';
}
