{
  description = "A Leptos project development environment with Fenix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    fenix.url = "github:nix-community/fenix/monthly";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
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
        buildInputs = with pkgs; [
          toolchain
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
              inherit buildInputs;
            };
      }

    );
}
