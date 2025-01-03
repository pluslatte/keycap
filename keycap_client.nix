# see this: https://github.com/NixOS/nixpkgs/issues/254369#issuecomment-2080460150
{ stdenvNoCC, yarn-berry, nodejs, cacert }:
stdenvNoCC.mkDerivation (finalAttrs: {
  pname = "keycap-client";
  version = "0.1.0";

  src = ./www/keycap-client/.;

  nativeBuildInputs = [
    yarn-berry
    nodejs
  ];

  yarnOfflineCache = stdenvNoCC.mkDerivation {
    name = "keycap-client-deps";
    nativeBuildInputs = [ yarn-berry ];
    inherit (finalAttrs) src;

    NODE_EXTRA_CA_CERTS = "${cacert}/etc/ssl/certs/ca-bundle.crt"; # What is this?

    supportedArchitectures = builtins.toJSON {
      os = [ "darwin" "linux" ];
      cpu = [ "arm" "arm64" "ia32" "x64" ];
      libc = [ "glibc" "musl" ];
    };

    configurePhase = ''
      runHook preConfigure

      export HOME="$NIX_BUILD_TOP"
      export YARN_ENABLE_TELEMETRY=0

      yarn config set enableGlobalCache false
      yarn config set cacheFolder $out
      yarn config set supportedArchitectures --json "$supportedArchitectures"

      runHook postConfigure
    '';

    buildPhase = ''
      runHook preBuild

      mkdir -p $out
      yarn install --immutable --mode skip-build

      runHook postBuild
    '';

    dontInstall = true;

    outputHashAlgo = "sha256";
    outputHash = "sha256-eojh68NzB+Hdf5KMATPP6pqqvb43XBTS51IxpV61Qio=";
    outputHashMode = "recursive";
  };

  configurePhase = ''
    runHook preConfigure

    export HOME="$NIX_BUILD_TOP"
    export YARN_ENABLE_TELEMETRY=0

    yarn config set enableGlobalCache false
    yarn config set cacheFolder $yarnOfflineCache

    runHook postConfigure
  '';

  buildPhase = ''
    runHook preBuild

    yarn install --immutable --immutable-cache
    yarn build
    yarn workspaces focus --all --production

    runHook postBuild
  '';

  installPhase = ''
    runHook preInstall

    mkdir -p $out/www/keycap-client/

    cp -r build $out/www/keycap-client/

    runHook postInstall
  '';

  fixupPhase = ''
    runHook preFixup

    runHook postFixup
  '';
})
