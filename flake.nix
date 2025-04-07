{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      # Unterstützte Systeme definieren
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];

      # Helfer-Funktion zur Generierung der Outputs für alle Systeme
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

      # Pkgs für jedes System
      nixpkgsFor = forAllSystems (system:
        import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        }
      );
    in {
      packages = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
        in {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "codesphere-cli";  # Name deines Projekts
            version = "0.1.0";         # Version deines Projekts

            src = ./.;

            cargoLock = {
              lockFile = ./Cargo.lock;
            };

            nativeBuildInputs = with pkgs; [
              pkg-config
              openssl
            ];

            buildInputs = with pkgs; [
              openssl
            ];

            meta = with pkgs.lib; {
              description = "CLI tool for managing Codesphere environment variables";
              homepage = "https://github.com/yourusername/codesphere-cli";  # Dein Repository
              license = licenses.mit;  # Oder deine gewählte Lizenz
              maintainers = with maintainers; [ "yourgithubusername" ];
              platforms = platforms.all;
            };
          };
        }
      );

      # Development shell
      devShells = forAllSystems (system:
        let
          pkgs = nixpkgsFor.${system};
        in {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${system}.default ];
            nativeBuildInputs = with pkgs; [
              rust-analyzer
              rustc
              cargo
              pkg-config
              openssl
            ];
          };
        }
      );
    };
}
