{ pkgs, makeRustPlatform, rust-bin, openssl, pkg-config, }:
let
  toolchain = rust-bin.stable.latest.default;
  rustPlatform = makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };
  keycapClient = pkgs.callPackage ./keycap_client.nix { };
in rustPlatform.buildRustPackage {
  pname = "keycap";
  version = "0.1.0";

  buildInputs = [ openssl keycapClient ];
  nativeBuildInputs = [ pkg-config ];

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  preInstall = ''
    echo "preInstall"
    mkdir -p $out
    cp -r ${keycapClient.outPath}/keycap-client $out/
  '';

  # postBuild = ''
  # '';
  # postInstall = ''
  # '';
  # postFixup = ''
  # '';
  # postConfigure = ''
  # '';

  # preBuild = ''
  # '';
  # preFixup = ''
  # '';
  # preConfigure = ''
  # '';
}
