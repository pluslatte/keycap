{
  self,
  pkgs,
  makeRustPlatform,
  rust-bin,
  openssl,
  pkg-config,
}:
let
  toolchain = rust-bin.stable.latest.default;

  rustPlatform = makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };

  keycapClient = pkgs.callPackage ./keycap_client.nix { };

  cargoEnvValExport = if self ? rev then "export GIT_HASH=${self.rev}" else "export GIT_HASH=dirty";
in
rustPlatform.buildRustPackage {
  name = "keycap";

  buildInputs = [
    openssl
    keycapClient
  ];
  nativeBuildInputs = [
    pkg-config
    pkgs.git
  ];

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  preBuild = ''
    ${cargoEnvValExport}
  '';

  preInstall = ''
    mkdir -p $out
    cp -r ${keycapClient.outPath}/keycap-client $out/
  '';
}
