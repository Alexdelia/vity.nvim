{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage {
  pname = "vity-nvim";
  version = "0.1.0";

  src = pkgs.lib.cleanSource ./.;

  cargoSha256 = "0";
  cargoLock.lockFile = ./Cargo.lock;

  postInstall = ''
    mv $out/lib $out/lua
	mv $out/lua/libvity_nvim.so $out/lua/vity.so
  '';
}
