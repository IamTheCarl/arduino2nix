# Arduino 2 Nix

This tool builds Arduino sketches as Nix derivations. The reason for doing this is for distributing firmware to robots. This is part of the [ROS Assistant](https://github.com/IamTheCarl/ros_assistant) experiment.

As of writing this, it is not in a functional state. I have posted it online in hopes I can get some feedback and assistance into its development.

# Nixifying your sketch

Run `arduino2nix generate` from the root of your Arduino sketch. Note that you need to have a valid `sketch.yaml` in your project. Arduino2Nix will read the content of that sketch.yaml and use it to produce a file called `Arduino.nix`.

Create a `default.nix` file in the root of your sketch and fill it with the following:
```nix
{ pkgs ? import <nixpkgs> { } }:
import ./Arduino.nix {
  inherit pkgs;

  # The only reason this `default.nix` is really necessary is to provide a name and version number to the derivation.
  pname = "blink";
  version = "1.0.0";
}
```

# Prior Work

* [NixDuino](https://github.com/boredom101/nixduino) - Does exactly what this project hopes to accomplish, except it doesn't support platform packages, so you can't build for 3rd party platforms such as the esp8266. It is also built off arduino-mk which [looks unmaintained to me](https://github.com/sudar/Arduino-Makefile/pulls). This project aims to work off [arduino-cli](https://github.com/arduino/arduino-cli) which is officially part of the Arduino project and very well mantained.