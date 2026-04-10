{
  description = "tmux-pomodoro - tmux plugin for Pomodoro timer status";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
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
