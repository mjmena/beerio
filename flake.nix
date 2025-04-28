{
  description = "A Leptos project development environment with Fenix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      fenix,
      crane,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        inherit (pkgs) lib;

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

        craneToolchain = (crane.mkLib pkgs).overrideToolchain toolchain;
        unfilteredRoot = ./.; # The original, unfiltered source
        src = lib.fileset.toSource {
          root = unfilteredRoot;
          fileset = lib.fileset.unions [
            # Default files from crane (Rust and cargo files)
            (craneToolchain.fileset.commonCargoSources unfilteredRoot)
            (lib.fileset.fileFilter (
              file:
              lib.any file.hasExt [
                "html"
                "css"
              ]
            ) unfilteredRoot)
            # Example of a folder for images, icons, etc
            (lib.fileset.maybeMissing ./public)
          ];
        };
        commonArgs = {
          inherit src;
          strictDeps = true;
          # We must force the target, otherwise cargo will attempt to use your native target
          CARGO_BUILD_TARGET = "wasm32-unknown-unknown";

          buildInputs =
            [
              # Add additional build inputs here
            ]
            ++ lib.optionals pkgs.stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libiconv
            ];
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneToolchain.buildDepsOnly (
          commonArgs
          // {
            # You cannot run cargo test on a wasm build
            doCheck = false;
          }
        );

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        # This derivation is a directory you can put on a webserver.
        beerio-app = craneToolchain.buildTrunkPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
            # The version of wasm-bindgen-cli here must match the one from Cargo.lock.
            # When updating to a new version replace the hash values with lib.fakeHash,
            # then try to do a build, which will fail but will print out the correct value
            # for `hash`. Replace the value and then repeat the process but this time the
            # printed value will be for the second `hash` below
            wasm-bindgen-cli = pkgs.buildWasmBindgenCli rec {
              src = pkgs.fetchCrate {
                pname = "wasm-bindgen-cli";
                version = "0.2.100";
                hash = "sha256-3RJzK7mkYFrs7C/WkhW9Rr4LdP5ofb2FdYGz1P7Uxog=";
                # hash = lib.fakeHash;
              };

              cargoDeps = pkgs.rustPlatform.fetchCargoVendor {
                inherit src;
                inherit (src) pname version;
                hash = "sha256-qsO12332HSjWCVKtf1cUePWWb9IdYUmT+8OPj/XP2WE=";
                # hash = lib.fakeHash;
              };
            };
          }
        );
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
          buildPhase = "trunk build --release --public-url /beerio";
          installPhase = ''
            cp dist/index.html dist/404.html
            cp -r dist $out
          '';
        };
        packages.craneWasmBundle = beerio-app;

      }

    );
}
