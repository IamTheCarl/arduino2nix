{ pkgs ? import <nixpkgs> { } }:
import ./Arduino.nix {
  inherit pkgs;

  pname = "blink";
  version = "1.0.0";
}
