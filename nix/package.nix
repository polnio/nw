{ pkgs, system, fenix }:
let
  fenix-packages = fenix.packages.${system};
  toolchain = fenix-packages.fromToolchainFile {
    file = ../rust-toolchain.toml;
    sha256 = "sha256-5yj6HOitbmoFFbdLiXy3Uu+rZVhHzJPhOqV5l6nuDZQ=";
  };
  rustPlatform = pkgs.makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };
in rustPlatform.buildRustPackage {
  pname = "nw";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
  nativeBuildInputs = with pkgs; [ pkg-config makeWrapper ];
  LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [ openssl ];
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  postFixup = ''
    wrapProgram $out/bin/nw --prefix LD_LIBRARY_PATH : ${
      pkgs.lib.makeLibraryPath [ pkgs.openssl ]
    }
  '';
}
