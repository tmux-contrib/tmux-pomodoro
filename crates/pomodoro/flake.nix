{
  description = "A simple pomodoro timer CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "pomodoro";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          # rusqlite with the `bundled` feature compiles SQLite from source
          # using the `cc` crate — no cmake needed, but we need a C compiler
          # which is provided by stdenv automatically.
          nativeBuildInputs = with pkgs; [ pkg-config ];

          meta = with pkgs.lib; {
            description = "A simple pomodoro timer CLI";
            license = licenses.mit;
            mainProgram = "pomodoro";
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
          ];
        };
      }
    );
}
