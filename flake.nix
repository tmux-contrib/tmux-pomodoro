{
  description = "tmux-pomodoro - a Pomodoro timer CLI and tmux plugin";

  nixConfig = {
    extra-substituters = [ "https://nixos-contrib.cachix.org" ];
    extra-trusted-public-keys = [ "nixos-contrib.cachix.org-1:WRyXCyMQGjYEEC87RdlW4ZbK/R3Ia4/WGYEoIP9X/o0=" ];
  };

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        rust-toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.name;
          inherit (manifest) version;
          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
          # nixpkgs builds the -sys crates against the libraries of the system, since its November release: libsqlite3-sys links this one
          buildInputs = [ pkgs.sqlite ];
          doCheck = false;
          meta = with pkgs.lib; {
            inherit (manifest) description;
            inherit (manifest) homepage;
            license = licenses.mit;
            mainProgram = manifest.name;
            platforms = platforms.unix;
          };
        };

        devShells.default = pkgs.mkShell {
          inherit (manifest) name;
          packages = [
            rust-toolchain
            pkgs.bash
            pkgs.tmux
            pkgs.bats
          ];
        };
      }
    );
}
