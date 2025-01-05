{
  self,
  pkgs,
  makeRustPlatform,
  rust-bin,
  openssl,
  pkg-config,
  keycapClient,
}:
let
  toolchain = rust-bin.stable.latest.default;

  rustPlatform = makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };

  cargoEnvValExport1 = if self ? rev then "export GIT_HASH=${self.rev}" else "export GIT_HASH=dirty";
  cargoEnvValExport2 = "export CLIENT_PATH=${keycapClient.outPath}/keycap-client";
in
rustPlatform.buildRustPackage {
  name = "keycap";

  buildInputs = [
    openssl
  ];
  nativeBuildInputs = [ pkg-config ];

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  preBuild = ''
    ${cargoEnvValExport1}
    ${cargoEnvValExport2}
  '';
}
