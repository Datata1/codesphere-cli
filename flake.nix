{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils"; 
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" ]; 
        };

      in {
        packages = {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "codesphere-cli";  
            version = "0.1.0";        

            src = ./.; 

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [
              pkg-config
            ];

            buildInputs = with pkgs; [
              openssl
            ];

            meta = with pkgs.lib; {
              description = "CLI tool for managing Codesphere environment variables";
              homepage = "https://github.com/Datata1/codesphere-cli"; 
              license = licenses.mit;  
              maintainers = with maintainers; [ "yourgithubusername" ]; 
            };
          };
        };

        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs; [
              rustToolchain
              rust-analyzer
              pkg-config
              openssl
            ];

            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";

            # CARGO_HOME = "${pkgs.buildDir}/cargo"; # Optional: Cargo Cache in temporärem Verzeichnis
            # RUSTFLAGS = "-D warnings"; # Optional: Warnungen als Fehler behandeln
          };
        };

        apps = {
          default = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/codesphere-cli"; 
          };
        };

      }
    );
}