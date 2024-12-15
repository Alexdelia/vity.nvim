{pkgs ? import <nixpkgs> {}}:
pkgs.rustPlatform.buildRustPackage {
  pname = "vity-nvim";
  version = "0.1.0";

  src = pkgs.lib.cleanSource ./.;

  cargoSha256 = pkgs.lib.fakeSha256;
  cargoLock = {
    lockFile = ./Cargo.lock;

    outputHashes = {
      "nvim-oxi-0.5.1" = "sha256-qq6uuGEkkDdnwyUnkAl1+E/DGd5bPuHSpwr74+EW0rE=";
    };
  };

  postInstall = ''
    mv $out/lib $out/lua
    mv $out/lua/libvity.so $out/lua/vity.so

    mkdir $out/colors
    echo "require('vity').load()" > $out/colors/vity.lua
  '';
}
