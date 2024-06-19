{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, fenix, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        # "aarch64-linux"
        # "x86_64-darwin"
        # "aarch64-darwin"
      ];
      perSystem = { pkgs, system, ... }:
        let
          # toolchain = fenix.packages.${system}.minimal.toolchain;
          fenix-packages = fenix.packages.${system};
          toolchain = fenix-packages.fromToolchainFile {
            file = ./rust-toolchain.toml;
            sha256 = "sha256-5yj6HOitbmoFFbdLiXy3Uu+rZVhHzJPhOqV5l6nuDZQ=";
          };
          rustPlatform = pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
        in {
          packages.default = rustPlatform.buildRustPackage {
            pname = "nw";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            nativeBuildInputs = with pkgs; [ pkg-config ];
            LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [ openssl ];
            PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          };
        };
    };
}
