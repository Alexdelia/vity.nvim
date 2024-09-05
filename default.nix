{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage {
  pname = "vity-nvim";
  version = "0.1.0";

  src = pkgs.lib.cleanSource ./.;

  cargoSha256 = pkgs.lib.fakeSha256;
  cargoLock.lockFile = ./Cargo.lock;

  postInstall = ''
       mv $out/lib $out/lua
    mv $out/lua/libvity.so $out/lua/vity.so

    mkdir $out/colors
    echo "require('vity').load()" > $out/colors/vity.lua
  '';
}
