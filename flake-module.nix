{ lib, mlib, options, ... }:
with builtins;
let
  inherit (lib) mkOption types;
  mkOption' = type: { _type = "option"; inherit type; };

  #? attrs -> [string] -> [attr]
  getAttrs = mlib.turn map (lib.flip getAttr);

  mkRustPlatform = options.rust.platformResolver.value;
  crates = options.rust.crates;

  mkDerivation = drv: pkgs: (mkRustPlatform pkgs).buildRustPackage drv;

  profileToPackageResolver =
    name: profile:
    addErrorContext "While resolving rust profile ${name}"
      (mkDerivation
        (lib.composeManyExtensions
          profile.enabledOverlays.value
          profile.recipe.value
        )
      );

  # profileToPackageResolver =
  #   lib.const                   # discard name
  #     (mkDerivation
  #       (mlib.trivial.fanout
  #         (mlib.turn lib.composeManyExtensions (getAttr "enabledOverlays"))
  #         (getAttr "recipe")
  #       )
  #     );

  #? { <name> :: lambda } -> a -> { <name> :: (lambda a) }
  callAttrsWith =
    lib.flip (mlib.turn mapAttrs (mlib.turn lib.const mlib.swap));

  mapPackages = callAttrsWith crates.packages.value;
in
{
  imports = [];

  options.rust = {
    platformResolver = mkOption {
      type = mkOption' mlib.types.function;

      default = getAttr "rustPlatform";
      apply = getAttr "resolver";

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
      apply = mlib.turn head attrValues;
    };

    crates.overlays = mkOption {
      type = types.attrsOf mlib.types.function;
      apply = attrValues;

      default.noHash =
        final: prev:
        { name = "${prev.name}-noHash"; cargoHash = ""; };
    };

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
              };

              enabledOverlays = mkOption {
                type = types.listOf (types.enum (attrNames crates.overlays.value));
                default = [];
                apply = getAttrs crates.overlays.value;
              };
            };
          }
        )
      );

      default = {
        # noHash = {
        #   recipe =
        #     crates.profiles.value.${crates.defaultProfile.value}.recipe;
        #   enabledOverlays = ["noHash"];
        # };
      };

      # apply = ;
    };

    crates.packages = mkOption {
      internal = true;
      type = types.lazyAttrsOf (types.functionTo types.package);
      default = mapAttrs (profileToPackageResolver) crates.profiles.value;
      # apply = x: x // { default = getAttr crates.defaultProfile.value x; };
    };
  };

  config = {
    perSystem =
      { system, self', pkgs, ... }:
      {
        packages = mapPackages pkgs;
      };
  };
}
