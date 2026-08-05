# SPDX-License-Identifier: MPL-2.0
# Non-flakes entry point: `nix-build` here, or import into configuration.nix.
#
#   nix-build            # builds ./result/bin/cosmic-ext-vpn-menu
#   import ./. { }       # from a NixOS module (pass your own pkgs)
{
  pkgs ? import <nixpkgs> { },
}:
pkgs.callPackage ./nix/package.nix { }
