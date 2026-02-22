{
  description = "A simple pomodoro timer CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        manifest = builtins.fromTOML (builtins.readFile ./Cargo.toml);
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.package.name;
          cargoLock.lockFile = ./Cargo.lock;
          inherit (manifest.package) version;
          src = ./.;

          # rusqlite with the `bundled` feature compiles SQLite from source
          # using the `cc` crate — no cmake needed, but we need a C compiler
          # which is provided by stdenv automatically.
          nativeBuildInputs = with pkgs; [ pkg-config ];

          meta = with pkgs.lib; {
            mainProgram = manifest.package.name;
            description = "A simple pomodoro timer CLI";
            license = licenses.mit;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            clippy
            rustc
            rustfmt
            rust-analyzer
          ];
        };
      }
    );
}
