{
  description = "Rust dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
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
        pkgs = import nixpkgs {
          inherit system;
        };

        rustVer = fenix.packages.${system}.stable;
        rustChan = rustVer.withComponents [
          "cargo"
          "clippy"
          "rust-src"
          "rustc"
          "rustfmt"
          "rust-analyzer"
        ];

      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs = [
            rustChan
          ]
          # Section added only on Linux systems
          ++ lib.optionals (!stdenv.isDarwin) [
            # Linker
            mold-wrapped
          ]
          # Section added only on Darwin (macOS) systems
          ++ lib.optionals stdenv.isDarwin [
            libiconv
          ];

          RUSTFLAGS = if stdenv.isDarwin then "" else "-C link-arg=-fuse-ld=mold";
        };
      }
    );
}
