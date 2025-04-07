{ pkgs ? import <nixpkgs> {} }:

let
  rustOverlay = import (builtins.fetchTarball {
    url = "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  });
  pkgs' = import pkgs { overlays = [ rustOverlay ]; };
in
pkgs'.rustPlatform.buildRustPackage {
  pname = "codesphere-cli";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  nativeBuildInputs = with pkgs'; [
    pkg-config
    openssl
  ];

  buildInputs = with pkgs'; [
    openssl
  ];

  meta = with pkgs'.lib; {
    description = "CLI tool for managing Codesphere environment variables";
    homepage = "https://github.com/Datata1/codesphere-cli"; # Dein Repository
    license = licenses.mit;
    maintainers = with maintainers; [ "Datata1" ];
    platforms = platforms.all;
  };
}
