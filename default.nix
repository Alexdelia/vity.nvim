{
  pkgs ? import <nixpkgs> {},
  rustToolchain,
}: let
  rustPlatform = pkgs.makeRustPlatform {
    cargo = rustToolchain;
    rustc = rustToolchain;
  };
in
  rustPlatform.buildRustPackage
  {
    pname = "vity-nvim";
    version = "0.1.0";

    src = pkgs.lib.cleanSource ./.;

    cargoSha256 = pkgs.lib.fakeSha256;
    cargoLock = {
      lockFile = ./Cargo.lock;

      outputHashes = {
        "nvim-oxi-0.6.0" = "sha256-zwXbsR6HgRH/mKx3Tn2f+ChNBlHWOkvjCGhaAJ/HjIk=";
      };
    };

    postInstall = ''
      mv $out/lib $out/lua
      mv $out/lua/libvity.so $out/lua/vity.so

      mkdir $out/colors
      echo "require('vity').load()" > $out/colors/vity.lua
    '';
  }
