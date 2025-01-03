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

  buildInputs = [ openssl ];
  nativeBuildInputs = [ pkg-config (pkgs.callPackage ./keycap_front.nix { }) ];

  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
}
