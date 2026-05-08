{
  description = "Discord bot for running chronomutants.";

  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/25.11";

    flake-module = { flake = false; url = ./flake-module.nix; };

    mlib.url = "github:MaxTheMooshroom/mlib.nix";
  };

  outputs = { self, flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } ({ lib, ... }: {
      systems = lib.systems.flakeExposed;

      imports = [ (import inputs.flake-module) ];

      rustCrate = (pkg: {
        pname = "chronobot";
        version = "0.1.0";

        src = self.outPath;

        cargoHash = "sha256-vPmrNdU4DXC1fxsmKYRQsFtfOifkEmYg2xV1EETOK1U=";

        meta = {
          description = "Discord bot for running chronomutants.";
          homepage = "https://github.com/MaxTheMooshroom/chronobot";
          # license = lib.licenses. # TODO:
        };
      });

      perSystem = { self', pkgs, ... }: {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            clippy
            rustfmt
          ];
        };
      };
    });
}
