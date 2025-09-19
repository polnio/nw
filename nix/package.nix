{
  # Functions
  lib,
  rustPlatform,
  makeWrapper,
  installShellFiles,

  # Packages
  nix-index,
  nix-output-monitor,
  nvd,
  openssl,
  pkg-config,

  # Options
  withUI ? false,
}:
let
  inherit (lib) makeLibraryPath makeBinPath;

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
    installShellFiles
  ];
  buildInputs = [
    nix-index
  ]
  ++ (lib.optionals withUI [
    nix-output-monitor
    nvd
  ]);

  buildFeatures = lib.optionals withUI [ "ui" ];
  checkFeatures = [ "ui" ];

  LD_LIBRARY_PATH = makeLibraryPath [ openssl ];
  PKG_CONFIG_PATH = "${openssl.dev}/lib/pkgconfig";
  RUST_SRC_PATH = rustPlatform.rustLibSrc;
  PATH = makeBinPath (
    lib.optionals withUI [
      nix-output-monitor
      nvd
    ]
  );
  GEN_ARTIFACTS = "artifacts";

  postFixup = ''
    wrapProgram $out/bin/${pname} --prefix LD_LIBRARY_PATH : ${LD_LIBRARY_PATH} --prefix PATH : ${PATH}
  '';
  postInstall = ''
    installManPage artifacts/${pname}.1
    installShellCompletion \
      --bash artifacts/${pname}.bash \
      --zsh artifacts/_nw \
      --fish artifacts/${pname}.fish \

  '';
}
