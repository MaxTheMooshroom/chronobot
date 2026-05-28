{
  description = "Discord bot for running chronomutants.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";

    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    flake-module = { flake = false; url = ./flake-module.nix; };

    mlib.url = "github:MaxTheMooshroom/mlib.nix";
  };

  outputs = { self, flake-parts, ... }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; specialArgs.mlib = inputs.mlib.lib; } (
      { lib, ... }:
      {
        systems = lib.systems.flakeExposed;

        imports = [
          inputs.mlib.flakeModules.perSystem-moduleArgs
          (import inputs.flake-module)
        ];

        rust.crates = {
          defaultProfile = "chronobot";
          profiles.chronobot.recipe.fixed-point =
            crate:
            {
              pname = "chronobot";
              version = "0.1.0";

              src = self.outPath;

              cargoHash = "sha256-0yD14H99inFqtylxTb1f8UcYtn6W6Wihnh+ip1NGX0U=";

              meta = {
                description = "Discord bot for running chronomutants.";
                homepage = "https://github.com/MaxTheMooshroom/chronobot";
                license = lib.licenses.mit;
              };
            };
        };

        perSystem =
          { system, self', pkgs, ... }:
          {
            devShells.default = pkgs.mkShell {
              packages = with pkgs; [
                # rustup
                cargo
                cargo-cache
                cargo-workspaces
                # rustc
                clippy
                rustfmt
              ];
            };
        };
      }
    );
}
