{
  description = "A Leptos project development environment with Fenix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix/monthly";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      fenix,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        # Fenix toolchain setup
        fenixPkgs = fenix.packages.${system};

        # Rust toolchain with components and WASM target
        toolchain = fenixPkgs.combine [
          fenixPkgs.latest.toolchain
          fenixPkgs.targets.wasm32-unknown-unknown.latest.rust-std
        ];

        # Packages needed for Leptos development
        packages = with pkgs; [
          rustywind
          leptosfmt
        ];
        nativeBuildInputs = with pkgs; [
          toolchain
          wasm-bindgen-cli
          trunk
          tailwindcss_4
        ];
      in
      {
        devShells.default =
          pkgs.mkShell.override
            {
              stdenv = pkgs.stdenvAdapters.useMoldLinker pkgs.clangStdenv;
            }
            {
              inherit packages;
              inherit nativeBuildInputs;
            };
        packages.githubPagesWasmBundle = pkgs.rustPlatform.buildRustPackage {
          inherit nativeBuildInputs;
          pname = "beerio";
          version = "0.0.1";
          src = pkgs.lib.cleanSource ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          buildPhase = "trunk build --release --public-url mjmena/beerio";
          installPhase = ''
            cp dist/index.html dist/404.html
            cp -r dist $out
          '';
        };
      }

    );
}
