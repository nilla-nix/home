
{ lib, config }:
let
  inherit (config) inputs;

  
in
{
  options.homes = lib.options.create {
      description = "Home-Manager homes to create.";
      default.value = { };
      type = lib.types.attrs.of (lib.types.submodule ({ config }: let
        homeForSystem = system: config.home-manager.lib.homeManagerConfiguration {
          pkgs = config.pkgs.${system};
          lib = config.pkgs.lib;
          extraSpecialArgs = { inherit system; } // config.args;
          modules = config.modules;
        };

        result = builtins.listToAttrs (builtins.map (system: {
          name = system;
          value = homeForSystem system;
        }) config.systems);

        home_name = config.__module__.args.dynamic.name;
        home_name_parts = builtins.match "([a-z][-a-z0-9]*)(@([-A-Za-z0-9]+))?(:([-_A-Za-z0-9]+))?" home_name;

        system = builtins.elemAt home_name_parts 4;

        systemProvided = system != null;
      in {
        options = {
          systems = lib.options.create {
            description = "The systems this home is valid on.";
            type = lib.types.list.of lib.types.string;
          } // (if systemProvided then {
            default.value = [ system ];
            writeable = false;
          } else {});
          
          args = lib.options.create {
            description = "Additional arguments to pass to home-manager modules.";
            type = lib.types.attrs.any;
            default.value = { };
          };

          home-manager = lib.options.create {
            description = "The home-manager input to use.";
            type = lib.types.raw;
            default.value =
              if inputs ? home-manager
              then inputs.home-manager.result
              else null;
          };

          pkgs = lib.options.create {
            description = "The Nixpkgs instance to use.";
            type = lib.types.raw;
            default.value =
              if
                inputs ? nixpkgs
              then
                inputs.nixpkgs.result
              else
                null;
          };

          modules = lib.options.create {
            description = "A list of modules to use for home-manager.";
            type = lib.types.list.of lib.types.raw;
            default.value = [ ];
          };

          result = lib.options.create {
            description = "The created Home Manager home for each of the systems.";
            type = lib.types.attrs.of lib.types.raw;
            writable = false;
            default.value = result;
          };
        };
      }));
    };

  config = {
    assertions = lib.attrs.mapToList
      (name: value: {
        assertion = !(builtins.isNull value.pkgs);
        message = "A Nixpkgs instance is required for the home-manager home \"${name}\", but none was provided and \"inputs.nixpkgs\" does not exist.";
      })
      config.homes;
  };
}
