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
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        keycapClient = pkgs.callPackage ./keycap_client.nix { };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            openssl.dev
            pkg-config
            nodejs
            yarn
            rust-bin.stable.latest.default
          ];
        };

        packages.default = pkgs.callPackage ./keycap_server.nix {
          inherit self;
          inherit keycapClient;
        };
      }
    );
}
