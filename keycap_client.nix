{ lib, stdenv, fetchYarnDeps, yarnConfigHook, yarnBuildHook, yarnInstallHook, nodejs, }:
stdenv.mkDerivation {
  pname = "keycap-client";
  version = "0.1.0";

  src = ./keycap-client/.;

  yarnOfflineCache = fetchYarnDeps {
    yarnLock = ./keycap-client/yarn.lock;
    hash = "sha256-eAvWa+v+GLvUg5KvGSkUSK861QGokkj8BhLg2h887Ds=";
  };

  nativeBuildInputs = [
    yarnConfigHook
    yarnInstallHook
    yarnBuildHook
    nodejs
  ];
}
