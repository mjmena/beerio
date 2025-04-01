{
  description = "A Leptos project development environment with Fenix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, fenix, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };

        # Fenix toolchain setup
        fenixPkgs = fenix.packages.${system};

        # Rust toolchain with components and WASM target
        rustToolchain = fenixPkgs.combine [
          fenixPkgs.stable.toolchain
          fenixPkgs.targets.wasm32-unknown-unknown.stable.rust-std
        ];

        # Packages needed for Leptos development
        leptosPackages = with pkgs; [
          rustToolchain
          cargo-leptos
          trunk # For building the frontend
          llvmPackages.bintools
        ];
      in
      {
        devShells.default = pkgs.mkShell
          {
            buildInputs = leptosPackages;
            CARGO_TARGET_WASM32_UNKNOWN_UNKNOWN_LINKER = "lld";

          };
      }
    );
}
