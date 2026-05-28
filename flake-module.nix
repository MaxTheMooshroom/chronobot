{ lib, mlib, options, ... }:
with builtins;
let
  inherit (lib) mkOption types;
  mkOption' = type: { _type = "option"; inherit type; };

  #? attrs -> [string] -> [attr]
  getAttrs = mlib.turn map (lib.flip getAttr);

  mkRustPlatform = options.rust.platformResolver.value;
  crates = options.rust.crates;

  buildRustPackage' = pkgs: (mkRustPlatform pkgs).buildRustPackage;

  profileToPackageResolver =
    name: profile:
    addErrorContext "While resolving rust profile ${name}"
      (lib.flip buildRustPackage'
        (lib.extends
          profile.enabledOverlays
          profile.recipe
        )
      );

  #? { <name> :: lambda } -> a -> { <name> :: (lambda a) }
  callAttrsWith =
    lib.flip (mlib.turn mapAttrs (mlib.turn lib.const mlib.swap));

  mapPackages = callAttrsWith crates.__packages.value;
in
{
  imports = [];

  options.rust = {
    platformResolver = mkOption {
      type = mlib.types.function;

      default = getAttr "rustPlatform";

      description = ''
        A function that gets the derivation-builder for each crate
        profile from a provided set of packages such as nixpkgs.
      '';
      example =
        lib.literalExpression "builtins.getAttr \"rustPlatform\"";
    };

    crates.defaultProfile =
      mkOption' (types.enum (attrNames crates.profiles.value));

    crates.recipes = mkOption {
      type = types.attrsOf (
        types.attrTag {
          fixed-point =
            mkOption' (types.functionTo (types.attrsOf types.unspecified));

          attrs = mkOption {
            type = types.attrsOf types.unspecified;
            apply = lib.const;
          };
        }
      );
      apply = mapAttrs (lib.const mlib.turn head attrValues);
    };

    crates.overlays = mkOption' (types.attrsOf mlib.types.function);

    crates.profiles = mkOption {
      type = types.lazyAttrsOf (
        types.submodule (
          { options, ... }:
          {
            options = {
              recipe = mkOption {
                type = types.attrTag {
                  fixed-point = mkOption' mlib.types.function;
                  name = mkOption {
                    type = (types.enum (attrNames crates.recipes.value));
                    #? <recipe-name> -> <recipe>
                    apply = lib.flip getAttr crates.recipes.value;
                  };
                };
                apply = mlib.turn head attrValues;
              };

              enabledOverlays = mkOption {
                type = types.listOf (types.enum (attrNames crates.overlays.value));
                default = [];
                #? [<recipe name>] -> <joined-overlay>
                apply =
                  mlib.turn
                    lib.composeManyExtensions
                    (getAttrs crates.overlays.value);
              };
            };
          }
        )
      );
    };

    crates.__packages = mkOption {
      internal = true;
      type = types.lazyAttrsOf (types.functionTo types.package);
      default = mapAttrs (profileToPackageResolver) crates.profiles.value;
      apply = x: x // { default = getAttr crates.defaultProfile.value x; };
    };
  };

  config = {
    rust.crates.overlays.noHash = final: prev: {
      name = "${prev.name or prev.pname or "defaultPackage"}-noHash";
      cargoHash = "";
    };

    rust.crates.profiles = {
      noHash = {
        recipe.fixed-point =
          crates.profiles.value.${crates.defaultProfile.value}.recipe;
        enabledOverlays = ["noHash"];
      };
    };

    perSystem =
      { system, self', pkgs, ... }:
      {
        packages = mapPackages pkgs;
      };
  };
}
