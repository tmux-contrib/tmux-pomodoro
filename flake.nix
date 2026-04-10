{
  description = "tmux-pomodoro - tmux plugin for Pomodoro timer status";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
<<<<<<< HEAD
    let
      targets = {
        "x86_64-linux" = "x86_64-unknown-linux-gnu";
        "aarch64-darwin" = "aarch64-apple-darwin";
        "x86_64-darwin" = "x86_64-apple-darwin";
      };
      hashes = {
        "x86_64-linux" = "sha256-WCL5ry5lp+cwjQJplaJVonLKzOGcbPBLxh2dt5Yomlk=";
        "aarch64-darwin" = "sha256-jR0pzIHjjcnL4kgHeAmhLCl1kqQqKnvzaQ2puibXpSk=";
        "x86_64-darwin" = "sha256-vBsVX9lfSPQCa9QGztN50gmzl0Xj2sFGtVIwA6epqLU=";
      };
    in
    flake-utils.lib.eachSystem (builtins.attrNames targets) (
||||||| parent of ab44390 (ci: migrate to nix-based builds and flake.lock management)
    let
      targets = {
        "x86_64-linux" = "x86_64-unknown-linux-gnu";
        "aarch64-darwin" = "aarch64-apple-darwin";
        "x86_64-darwin" = "x86_64-apple-darwin";
      };
      hashes = {
        "x86_64-linux" = "sha256-nyeCeWNf58oDlu1wtWF/ZbhvtKG38JzZqrKAmswgGgw=";
        "aarch64-darwin" = "sha256-LBKfP/zgpWHB+rcXsfFGcUwzNwSEM5TaAaE/atwKLAg=";
        "x86_64-darwin" = "sha256-/KS+Vain0bImm6ms4v4jRLzfE5hYKDY7Lqt9IjQOFVg=";
      };
    in
    flake-utils.lib.eachSystem (builtins.attrNames targets) (
=======
    flake-utils.lib.eachDefaultSystem (
>>>>>>> ab44390 (ci: migrate to nix-based builds and flake.lock management)
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShells.default = pkgs.mkShell {
          name = "tmux-pomodoro";
          packages = with pkgs; [
            bash
            tmux
            bats
          ];
        };
      }
    );
}
