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

        packages.default = pkgs.callPackage ./default.nix { inherit rustPlatform; };

        apps.default = flake-utils.lib.mkApp {
          drv = self.packages.${system}.default;
        };
      }
    )
    // {
      overlays.default = final: prev: {
        anki-weblio = final.callPackage ./default.nix { };
      };
    };
}
