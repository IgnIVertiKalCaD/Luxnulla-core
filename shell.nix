{ pkgs ? import <nixpkgs> {} }:

let
  myBuildInputs = with pkgs; [
  ];
in
pkgs.mkShell {
  shellHook = ''
    rustup default nightly

    export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath myBuildInputs}:$LD_LIBRARY_PATH"

    echo "Welcome to the Nix-shell for your Zed project!"
  '';
}
