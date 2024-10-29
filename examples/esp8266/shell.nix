{ pkgs ? import <nixpkgs> { }, ... }:
pkgs.mkShell {
  buildInputs = [
    pkgs.arduino-cli
  ];
}
