{ pkgs, makeRustPlatform, rust-bin, openssl, pkg-config, }:
let
  toolchain = rust-bin.stable.latest.default;
  rustPlatform = makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };
in
rustPlatform.buildRustPackage {
  pname = "keycap";
  version = "0.1.0";

  buildInputs = [ openssl (pkgs.callPackage ./keycap_front.nix { }) ];
  nativeBuildInputs = [ pkg-config ];

  postFixup = ''
    mkdir -p $out/www
    cp -r www/keycap-client $out/www
  '';

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
