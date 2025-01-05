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
  nativeBuildInputs = [ pkg-config pkgs.git ];

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  # postBuild = ''
  #   echo "postBuild"
  #   ls -la keycap-client
  # '';
  # postInstall = ''
  #   echo "postInstall"
  #   ls -la keycap-client
  # '';
  # postFixup = ''
  #   echo "postFixup"
  #   ls -la keycap-client
  # '';
  # postConfigure = ''
  #   echo "postConfigure"
  #   ls -la keycap-client
  # '';

  # preBuild = ''
  #   echo "preBuild"
  #   ls -la keycap-client
  # '';
  preInstall = ''
    echo "preInstall"
    ls -la keycap-client
    ls -la ${keycapClient.outPath}
    mkdir -p $out
    cp -r ${keycapClient.outPath}/keycap-client $out/
  '';
  # preFixup = ''
  #   echo "preFixup"
  #   ls -la keycap-client
  # '';
  # preConfigure = ''
  #   echo "preConfigure"
  #   ls -la keycap-client
  # '';
}
