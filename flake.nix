{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      fenix,
      nixpkgs,
      flake-utils,
    }:

    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };

        toolchain = (
          pkgs.fenix.complete.withComponents [
            "cargo"
            "clippy"
            "rust-src"
            "rustc"
            "rustfmt"
          ]
        );
        rustPlatform = pkgs.makeRustPlatform {
          cargo = toolchain;
          rustc = toolchain;
        };
      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            packages = [
              toolchain
              pkg-config
              openssl
            ];
            LD_LIBRARY_PATH = lib.makeLibraryPath [ openssl ];
          };

        packages.default =
          with pkgs;
          rustPlatform.buildRustPackage {
            pname = "anki-weblio";
            version = "0.1.0";

            buildInputs = [ openssl ];
            nativeBuildInputs = [ pkg-config ];

            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };
      }
    );
}
