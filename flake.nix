{
  description = "A devShell example";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        metadata = (pkgs.lib.importTOML ./Cargo.toml).package;
      in
      {
        packages = rec {
          default = rsa;

          rsa = pkgs.rustPlatform.buildRustPackage {
            pname = metadata.name;
            version = metadata.version;
            cargoLock.lockFile = ./Cargo.lock;
            src = pkgs.lib.cleanSource ./.;
          };
        };
        devShells.default = pkgs.mkShell {
          name = "Rust Dev Shell";
          buildInputs = with pkgs; [
            fd
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
              targets = [ "arm-unknown-linux-gnueabihf" ];
            })
            rust-analyzer
            pkg-config
            gcc
          ];

          shellHook = ''
            alias find=fd
          '';
        };
      }
    );
}
