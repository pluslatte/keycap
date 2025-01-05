{
  stdenv,
  fetchYarnDeps,
  yarnConfigHook,
  yarnBuildHook,
  yarnInstallHook,
  nodejs,
}:
stdenv.mkDerivation {
  name = "keycap-client";

  src = ./keycap-client;

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

  postInstall = ''
    ls -la
    mkdir -p $out/keycap-client
    mv build/* $out/keycap-client/
  '';
}
