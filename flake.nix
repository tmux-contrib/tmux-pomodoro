{
  description = "tmux-pomodoro - tmux plugin for Pomodoro timer status";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
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
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        manifest = builtins.fromTOML (builtins.readFile ./crates/pomodoro/Cargo.toml);
        pname = manifest.package.name;
        version = manifest.package.version;
      in
      {
        packages.default = pkgs.stdenvNoCC.mkDerivation {
          inherit pname version;

          src = pkgs.fetchurl {
            url = "https://github.com/tmux-contrib/tmux-pomodoro/releases/download/v${version}/${pname}-${targets.${system}}";
            hash = hashes.${system};
          };

          phases = [ "installPhase" ];

          installPhase = ''
            mkdir -p $out/bin
            cp $src $out/bin/${pname}
            chmod +x $out/bin/${pname}
          '';

          meta = with pkgs.lib; {
            mainProgram = pname;
            description = "A simple pomodoro timer CLI";
            license = licenses.mit;
            platforms = builtins.attrNames targets;
          };
        };

        devShells.default = pkgs.mkShell {
          name = "tmux-pomodoro";
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
      }
    );
}
