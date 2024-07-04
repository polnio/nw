{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs@{ flake-parts, fenix, self, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
        # "aarch64-linux"
        # "x86_64-darwin"
        # "aarch64-darwin"
      ];
      perSystem = { pkgs, system, ... }: {
        packages.default =
          import ./nix/package.nix { inherit pkgs system fenix; };
      };
      flake = {
        nixosModules.default = import ./nix/nixos.nix self;
        homeManagerModules.default = import ./nix/hm.nix self;
      };
    };
}
