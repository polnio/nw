{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      flake-parts,
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
        {
          packages = {
            default = pkgs.callPackage ./nix/package.nix { };
            without-ui = pkgs.callPackage ./nix/package.nix {
              withUI = false;
            };
            with-ui = pkgs.callPackage ./nix/package.nix {
              withUI = true;
            };
          };
        };
      flake = {
        nixosModules.default = import ./nix/nixos.nix self;
        homeManagerModules.default = import ./nix/hm.nix self;
      };
    };
}
