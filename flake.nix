{
  description = "A Leptos project development environment with Fenix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix/monthly";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = import nixpkgs { inherit system; };

          # Fenix toolchain setup
          fenixPkgs = fenix.packages.${system};

          # Rust toolchain with components and WASM target
          toolchain = fenixPkgs.combine [
            fenixPkgs.latest.toolchain
            fenixPkgs.targets.wasm32-unknown-unknown.latest.rust-std
          ];
        in
        {
          devShells.default = pkgs.mkShell {
            buildInputs = with pkgs; [
              toolchain
              binaryen
              llvmPackages.bintools
              cargo-leptos
              cargo-generate
              tailwindcss_4
            ];
            packages = [ pkgs.just pkgs.bacon pkgs.watchexec ];

            CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";

            LEPTOS_OUTPUT_NAME = "beerio";
            LEPTOS_SITE_ROOT = "site";
            LEPTOS_SITE_PKG_DIR = "pkg";
            LEPTOS_SITE_ADDR = "0.0.0.0:3000";
            LEPTOS_RELOAD_PORT = "3001";
          };
        }
      );
}
