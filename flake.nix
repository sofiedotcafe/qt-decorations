{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
    systems.url = "github:nix-systems/default";
    hercules-ci-effects.url = "github:hercules-ci/hercules-ci-effects";

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;

      imports = [
        inputs.hercules-ci-effects.flakeModule
      ];

      flake.homeModules = rec {
        default = import ./home-manager.nix inputs;
        qt-decorations = default;
      };

      perSystem =
        { pkgs, ... }:
        {
          packages.default = pkgs.callPackage ./package.nix { };

          devShells.default = pkgs.mkShell {
            packages = [
              pkgs.cargo
              pkgs.rustc
              pkgs.clippy
            ];

            nativeBuildInputs = [
              pkgs.pkg-config
              pkgs.llvmPackages.libclang
            ];

            buildInputs = [
              pkgs.qt6.qtbase
              pkgs.dbus
            ];

            shellHook = ''
              export PATH="${pkgs.qt6.qtbase}/libexec:$PATH"
              export LIBCLANG_PATH="${pkgs.llvmPackages.libclang.lib}/lib"
            '';
          };
        };
    };
}
