{
  description = "tmux-pomodoro - tmux plugin for Pomodoro timer status";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        manifest = builtins.fromTOML (builtins.readFile ./crates/pomodoro/Cargo.toml);
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = manifest.package.name;
          cargoLock.lockFile = ./crates/pomodoro/Cargo.lock;
          inherit (manifest.package) version;
          src = ./crates/pomodoro;
          nativeBuildInputs = with pkgs; [ pkg-config ];
          meta = with pkgs.lib; {
            mainProgram = manifest.package.name;
            description = "A simple pomodoro timer CLI";
            license = licenses.mit;
          };
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            bash
            tmux
            bats
          ];
          buildInputs = with pkgs; [
            cargo
            clippy
            rustc
            rustfmt
            rust-analyzer
          ];
        };
      });
}
