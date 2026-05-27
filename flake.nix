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

      rustCrate =
        crate:
        {
          pname = "chronobot";
          version = "0.1.0";

          src = self.outPath;

          cargoHash = "sha256-mu2nYX38M9QcT4JCr2oVJeeLCpB3hbK2k+u/NsYM/eA=";

          meta = {
            description = "Discord bot for running chronomutants.";
            homepage = "https://github.com/MaxTheMooshroom/chronobot";
            license = lib.licenses.mit;
          };
        };

      perSystem =
        { system, self', pkgs, ... }:
        {
          devShells.default = pkgs.mkShell {
            packages = with pkgs; [
              # rustup
              cargo
              # rustc
              clippy
              rustfmt
            ];
          };
      };
    });
}
