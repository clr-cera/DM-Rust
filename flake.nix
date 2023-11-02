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
      in
      with pkgs;
      {
        devShells.default = mkShell {
          name = "Rust Dev Shell";
          buildInputs = [
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
