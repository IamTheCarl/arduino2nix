# Auto-generated with arduino2nix. Do not modify.
# Edit your sketch.yaml and then run `arduino2nix generate` to update this file.
{ pkgs ? <nixpkgs> { }, pname, version }:
let
  esp-8266-esp-8266-v3-1-2 = (pkgs.fetchurl {
    url =
      "https://github.com/esp8266/Arduino/releases/download/3.1.2/esp8266-3.1.2.zip";
    sha256 = "b3f47686d7783c120c2f10bf82788f921c53db8642cc87012599abb6e335b182";
  });
  sketch_yaml = pkgs.writeText "sketch.json" ''
    profiles:
      blink:
        fqbn: esp8266:esp8266:nodemcuv3
        platforms:
        - platform: esp8266:esp8266 (3.1.2)
          platform_index_url: file://${esp-8266-esp-8266-v3-1-2}
        libraries: []

  '';
in pkgs.stdenv.mkDerivation {
  inherit pname version;
  src = ./.;
  buildInputs = [ pkgs.arduino-cli ];
  postUnpack = ''
    rm -f $sourceRoot/sketch.yaml
    ln -s ${sketch_yaml} $sourceRoot/sketch.yaml
    cat $sourceRoot/sketch.yaml
  '';
  buildPhase = ''
    arduino-cli compile --profile ${pname} --output-dir output
  '';
  installPhase = ''
    cp output/${pname}.ino.elf $out/payload.elf
  '';
}
