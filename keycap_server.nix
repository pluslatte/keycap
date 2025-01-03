{ pkgs, makeRustPlatform, rust-bin, openssl, pkg-config, }:
let
  toolchain = rust-bin.stable.latest.default;
  rustPlatform = makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };
in
rustPlatform.buildRustPackage
{
  pname = "keycap";
  version = "0.1.0";

  buildInputs = [ openssl (pkgs.callPackage ./keycap_client.nix { }) ];
  nativeBuildInputs = [ pkg-config ];

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;

  postFixup = ''
    cp -r keycap-client $out/
  '';
}
