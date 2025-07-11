# Free SCADA: A general-porpoise SCADA solution.

Free SCADA is a free, open source SCADA system for everything you
need, from smart houses, to country-wide power systems.

It is built entirely in Rust to ensure safety, reliability and
performance.

Everything is...

For free SCADA development!

## Setup

### Non-NixOS systems

To setup the project, first install Nix on your system.
Then, install Devenv.
Next, install Direnv.
Lastly, install Git.

After that, run:

```shell
git clone https://github.com/freeSCADA/free-scada
direnv allow free-scada
cd free-scada
```

Direnv should build the dev environment with this.
Make sure your internet connection is up, since it will download all the required dependencies for the project.

### NixOS

The process is basically the same as above.
Put this into your `configuration.nix`:

```nix
# configuration.nix
programs.nix-ld.enable = true;
nix.settings = {
  substituters = [
    "https://devenv.cachix.org"
  ];
  trusted-substituters = [
    "https://devenv.cachix.org"
  ];
  trusted-public-keys = [
    "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
  ];
  trusted-users = [
    "root"
    "your_username"
    "@wheel"
  ];
};
environment.systemPackages = with pkgs; [
  devenv
  direnv
  git
];
```

Notice that you will need to patch some dependencies with `nix-ld` later.

After that, run:

```shell
git clone https://github.com/freeSCADA/free-scada
direnv allow free-scada
cd free-scada
```

## Running

```shell
cargo run
```

## Contributors

- [@jakkunight](https://github.com/jakkunight)
- [@israelpaniaguadev](https://github.com/israelpaniaguadev)
- [@freeSCADA](https://github.com/freeSCADA)
