{
  description = "A Catppuccin-style client-side decoration plugin for Qt";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    hercules-ci-effects.url = "github:hercules-ci/hercules-ci-effects";

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
      ];
      imports = [
        inputs.pre-commit-hooks.flakeModule
        inputs.hercules-ci-effects.flakeModule
      ];

      flake.homeModules = rec {
        default = import ./nix/homeModule.nix inputs;
        qt-decorations = default;
      };

      perSystem =
        {
          config,
          pkgs,
          self',
          lib,
          ...
        }:
        {
          devShells.default = pkgs.mkShell {
            packages = lib.attrValues self'.packages;
            shellHook = ''
              ${config.pre-commit.installationScript}
            '';
          };

          pre-commit = {
            check.enable = true;
            settings = {
              hooks = {
                clang-format.enable = true;
                nixfmt-rfc-style.enable = true;
                deadnix.enable = true;
                statix.enable = true;
                commitizen.enable = true;
                typos.enable = true;
                shellcheck.enable = true;
                markdownlint.enable = true;
              };
            };
          };
          packages = {
            qcatppuccindecorations = pkgs.callPackage ./nix/package.nix {
              version = "0-unstable-${builtins.toString inputs.self.lastModified}";
            };
          };
        };
    };
}
