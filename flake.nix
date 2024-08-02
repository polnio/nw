{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      flake-parts,
      fenix,
      self,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        # "aarch64-linux"
        # "x86_64-darwin"
        # "aarch64-darwin"
      ];
      perSystem =
        { pkgs, system, ... }:
        let
          fenix-pkgs = fenix.packages.${system};
        in
        {
          packages = {
            default = pkgs.callPackage ./nix/package.nix { inherit fenix-pkgs; };
            without-ui = pkgs.callPackage ./nix/package.nix {
              inherit fenix-pkgs;
              withUi = false;
            };
            with-ui = pkgs.callPackage ./nix/package.nix {
              inherit fenix-pkgs;
              withUi = true;
            };
          };
        };
      flake = {
        nixosModules.default = import ./nix/nixos.nix self;
        homeManagerModules.default = import ./nix/hm.nix self;
      };
    };
}
