{
  description = "protoc-types - the protoc.contrib proto packages, with their Go and Rust bindings";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          name = "protoc-types";
          # `buf` regenerates the Go bindings and `protoc` supplies the
          # well-known types tests/generated.rs regenerates the Rust ones
          # against. Neither is needed to build: the generated code is
          # committed.
          packages = [
            pkgs.go
            pkgs.buf
            pkgs.protobuf
            rust-toolchain
          ];
        };
      }
    );
}
