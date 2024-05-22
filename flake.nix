{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rust = (pkgs.rust-bin.stable.latest.default.override {
          extensions = [
            "cargo"
            "clippy"
            "rust-src"
            "rust-analyzer"
            "rustc"
            "rustfmt"
          ];
          targets = [ "wasm32-unknown-unknown" ];
        });
      in
      with pkgs;
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs;[
            pkg-config
            alsaLib
            rust
            trunk
          ];
        };
      }
    );
}
