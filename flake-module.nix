{ inputs, lib, options, ... }:
let
  inherit (lib) mkOption types;
  mlib = inputs.mlib.lib;

  noHash = final: prev: { cargoHash = ""; };
in
{
  imports = [
    { options = {
        rustCrate = mkOption {
          type = mlib.types.function;
        };
      };
    }
  ];

  config = {
    perSystem = { self', pkgs, ... }:
      let
        crate = pkgs.rustPlatform.buildRustPackage options.rustCrate.value;
        pname = crate.pname;
      in {
        packages = {
          default = pkgs.hello;
          ${pname} = crate;

          noHash =
            pkgs.rustPlatform.buildRustPackage
              (lib.extends noHash options.rustCrate.value);
        };
      };
  };
}
