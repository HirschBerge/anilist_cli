{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    naersk.url = "github:nix-community/naersk";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {
    self,
    naersk,
    nixpkgs,
    rust-overlay,
  }: let
    overlays = [
      rust-overlay.overlays.default
      (final: prev: {
        rustToolchain = let
          rust = prev.rust-bin;
        in
          if builtins.pathExists ./rust-toolchain.toml
          then rust.fromRustupToolchainFile ./rust-toolchain.toml
          else if builtins.pathExists ./rust-toolchain
          then rust.fromRustupToolchainFile ./rust-toolchain
          else rust.stable.latest.default;
      })
    ];
    supportedSystems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forEachSupportedSystem = f:
      nixpkgs.lib.genAttrs supportedSystems (system:
        f {
          pkgs = import nixpkgs {inherit overlays system;};
        });
  in {
    devShells = forEachSupportedSystem ({pkgs}: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          rustToolchain
          openssl
          pkg-config
          cargo-deny
          cargo-edit
          cargo-watch
          rust-analyzer
          bacon
        ];
      };
    });
    packages = forEachSupportedSystem ({pkgs}: let
      naersklib = pkgs.callPackage naersk {};
    in {
      default = naersklib.buildPackage {
        pname = "anilist_cli";
        version = "0.2.0"; # Should match your Cargo.toml version
        # The `src` is the flake's own directory
        src = self;
        nativeBuildInputs = with pkgs; [pkg-config];
        buildInputs = with pkgs; [openssl];
      };
    });
  };
}
