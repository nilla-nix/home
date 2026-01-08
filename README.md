# 🍦 Nilla Home

> Work with Home Manager configurations in [Nilla](https://github.com/nilla-nix/nilla) projects with ease.

## Integration with Nilla CLI

Nilla Home integrates with the [Nilla CLI](https://github.com/nilla-nix/cli) through its external subcommand mechanism. When you run `nilla home`, the Nilla CLI looks for a binary named `nilla-home` in your PATH and executes it with the remaining arguments.

### How It Works

The Nilla CLI supports external subcommands via the `external_subcommand` mechanism. When you run `nilla <subcommand>`, it searches for a binary named `nilla-<subcommand>` in your PATH. Once `nilla-home` is installed (using one of the methods below), it will be available as `nilla-home` and can be used via the Nilla CLI.

### Commands

Once installed, you can use it via the Nilla CLI:

```bash
# Build a Home Manager configuration
nilla home build <specifier>

# Build and switch to a configuration
nilla home switch <specifier>

# Pass additional nix build options
nilla home build <specifier> -- --builders "ssh://remote x86_64-linux"
nilla home switch <specifier> -- --builders "ssh://remote x86_64-linux"
```

The `<specifier>` follows the format `{username}[@hostname][:system]`, for example:

- `user` - for the current user on the current hostname
- `user@hostname` - for a specific user on a specific hostname
- `user@hostname:x86_64-linux` - with an explicit system architecture

## Install with Nilla

You can add Nilla Home to your Nilla project and access using the following code:

```nix
# In any module of your project.
{ config }:
let
    nilla-home-package = config.inputs.nilla-home.packages.nilla-home.x86_64-linux;
in
{
    config = {
        inputs.nilla-home.src = builtins.fetchTarball {
            url = "https://github.com/nilla-nix/home/archive/main.tar.gz";
            sha256 = "0000000000000000000000000000000000000000000000000000";
        };

        # Do something with the package.
    };
}
```

## Install without Flakes

You can install Nilla Home in your NixOS, home-manager, or nix-darwin configuration.

```nix
# configuration.nix
{ pkgs, ... }:
let
  nilla-home = import (builtins.fetchTarball {
    url = "https://github.com/nilla-nix/home/archive/main.tar.gz";
    sha256 = "0000000000000000000000000000000000000000000000000000";
  });
  nilla-home-package = nilla-home.packages.nilla-home.result.${pkgs.system};
in
{
  environment.systemPackages = [
    nilla-home-package
  ];
}
```

## Install with Flakes

You can add Nilla Home as a Flake input.

```nix
# flake.nix
{
  inputs = {
    nilla-home.url = "github:nilla-nix/home";
  };

  outputs = { nilla-home, ... }:
    let
      nilla-home-package = nilla-home.packages.x86_64-linux.nilla-home;
    in
      # Do something with the package.
      {};
}
```

## Run with Flakes

You can run Nilla Home directly via Flakes.

```bash
# Place any arguments you want to provide to Nilla Home after the --
nix run github:nilla-nix/home -- --help
```
