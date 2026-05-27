{ inputs, lib, options, ... }:
let
  inherit (lib) mkOption types;
  mlib = inputs.mlib.lib;

  noHash = final: prev: { cargoHash = ""; };
in
{
  imports = [
    {
      options.rustCrate = mkOption {
        type = mlib.types.function;
      };
    }
  ];

  config = {
    perSystem = { system, self', pkgs, ... }:
      let
        rustPlatform = pkgs.rustPlatform;
        crate = rustPlatform.buildRustPackage options.rustCrate.value;
        pname = crate.pname;
      in {
        packages = {
          default = self'.packages.${pname};
          ${pname} = crate;

          noHash =
            pkgs.rustPlatform.buildRustPackage
              (lib.extends noHash options.rustCrate.value);
        };
      };
  };
}
