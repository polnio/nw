{
  # Inputs
  fenix-pkgs,

  # Functions
  lib,
  makeRustPlatform,
  makeWrapper,

  # Packages
  nix-output-monitor,
  nvd,
  openssl,
  pkg-config,

  # Options
  withUI ? false,
}:
let
  inherit (lib) makeLibraryPath makeBinPath;

  toolchain = fenix-pkgs.fromToolchainFile {
    file = ../rust-toolchain.toml;
    sha256 = "sha256-5yj6HOitbmoFFbdLiXy3Uu+rZVhHzJPhOqV5l6nuDZQ=";
  };
  rustPlatform = makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };

  cargoConfig = builtins.fromTOML (builtins.readFile ../Cargo.toml);
in
rustPlatform.buildRustPackage rec {
  pname = cargoConfig.package.name;
  version = cargoConfig.package.version;
  src = ../.;
  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [
    pkg-config
    makeWrapper
  ];
  buildInputs = lib.optionals withUI [
    nix-output-monitor
    nvd
  ];

  buildFeatures = lib.optionals withUI [ "ui" ];
  checkFeatures = [ "ui" ];

  LD_LIBRARY_PATH = makeLibraryPath [ openssl ];
  PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig";
  PATH = makeBinPath (
    lib.optionals withUI [
      nix-output-monitor
      nvd
    ]
  );
  postFixup = ''
    wrapProgram $out/bin/nw --prefix LD_LIBRARY_PATH : ${LD_LIBRARY_PATH} --prefix PATH : ${PATH}
  '';
}
