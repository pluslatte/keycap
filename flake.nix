{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    { nixpkgs
    , flake-utils
    , rust-overlay
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            openssl
            pkg-config
            rust-bin.stable.latest.default
          ];
        };

        packages = {
          default = pkgs.symlinkJoin {
            name = "keycap";
            paths = [
              (pkgs.callPackage ./keycap_server.nix { })
              (pkgs.callPackage ./keycap_front.nix { })
            ];
          };
        };
      }
    );
}
