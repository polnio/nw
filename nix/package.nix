{
  pkgs,
  system,
  fenix,
  withUi ? false,
}:
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

  libraries = pkgs.lib.makeLibraryPath (with pkgs; [ openssl ]);
in
rustPlatform.buildRustPackage {
  pname = "nw";
  version = "0.1.0";
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;
  nativeBuildInputs = with pkgs; [
    pkg-config
    makeWrapper
  ];
  buildInputs = pkgs.lib.optionals withUi (
    with pkgs;
    [
      nix-output-monitor
      nvd
    ]
  );
  buildFeatures = pkgs.lib.optional withUi "ui";
  checkFeatures = [ "ui" ];
  LD_LIBRARY_PATH = libraries;
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
  postFixup = ''
    wrapProgram $out/bin/nw --prefix LD_LIBRARY_PATH : ${libraries}
  '';
}
