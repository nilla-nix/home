
{ lib, config }:
let
  inherit (config) inputs;
  homes-type = import ./homes-type.nix { inherit lib config; };
in
{
  options.homes = lib.options.create {
    description = "Home-Manager homes to create.";
    default.value = { };
    type = homes-type;
  };
}
