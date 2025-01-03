{ lib, stdenv, fetchYarnDeps, yarnConfigHook, yarnBuildHook, yarnInstallHook, nodejs, }:
stdenv.mkDerivation {
  pname = "keycap-client";
  version = "0.1.0";

  src = ./www/keycap-client/.;

  yarnOfflineCache = fetchYarnDeps {
    yarnLock = ./front/yarn.lock;
    # hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
  };

  nativeBuildInputs = [
    yarnConfigHook
    yarnBuildHook
    yarnInstallHook
    nodejs
  ];
}
