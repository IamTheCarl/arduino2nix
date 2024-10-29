# An Arduino sketch example

You can build an Arduino sketch with arduino-cli and deploy it with rass.
Currently building the sketch with rass is not supported. A lot of work needs to be done to get arduino-cli to be able to use 3rd party platforms (necessary for the esp8266).

# Building

Run `nix-shell --run './build.sh'`. The output of arduino-cli will be placed in a newly created directory called output. After that you can run `nix-build` to create the derivation.