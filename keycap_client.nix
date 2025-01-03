{ lib, stdenv, fetchYarnDeps, yarnConfigHook, yarnBuildHook, yarnInstallHook, nodejs, }:
stdenv.mkDerivation {
  pname = "keycap-client";
  version = "0.1.0";

  src = ./www/keycap-client/.;

  yarnOfflineCache = fetchYarnDeps {
    yarnLock = ./www/keycap-client/yarn.lock;
    hash = "sha256-eAvWa+v+GLvUg5KvGSkUSK861QGokkj8BhLg2h887Ds=";
  };

  nativeBuildInputs = [
    yarnConfigHook
    yarnBuildHook
    yarnInstallHook
    nodejs
  ];
}
