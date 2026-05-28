{
  description = "Discord bot for running chronomutants.";

  inputs = {
    # Using branch `nixos-25.11` instead of tag `25.11` because
    # rust builds in the current (as of writing) nixpkgs release
    # are broken by the crates.io UA changes. nixos-* branches
    # still receive maintainance after release, unlike tagged
    # releases.
    #
    # For more information, see:
    # https://github.com/rust-lang/crates.io/issues/13482
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";

    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    flake-module = { flake = false; url = ./flake-module.nix; };

    mlib.url = "github:MaxTheMooshroom/mlib.nix";
    mlib.inputs.flake-parts.follows = "flake-parts";
  };

  outputs = { self, ... }@inputs:
    inputs.mlib.lib.mkFlake { inherit inputs; } (
      { config, lib, mlib, ... }:
      {
        systems = lib.systems.flakeExposed;

        imports = [ (import inputs.flake-module) ];

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
                cargo
                cargo-workspaces
                clippy
                rustfmt
              ];
            };
        };
      }
    );
}
