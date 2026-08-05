# SPDX-License-Identifier: MPL-2.0
{
  description = "COSMIC VPN Menu — a VPN management applet for the COSMIC desktop";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { self, nixpkgs }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});

      # Libraries libcosmic (winit/wayland/wgpu) dlopens or links at runtime.
      runtimeLibs =
        pkgs: with pkgs; [
          wayland
          libxkbcommon
          vulkan-loader
          libGL
          fontconfig
          freetype
          expat
        ];
    in
    {
      packages = forAllSystems (pkgs: rec {
        # The derivation is defined once in ./nix/package.nix and shared with the
        # non-flakes default.nix.
        cosmic-ext-vpn-menu = pkgs.callPackage ./nix/package.nix { };
        default = cosmic-ext-vpn-menu;
      });

      # Add `cosmic-ext-vpn-menu` to nixpkgs, e.g. via `nixpkgs.overlays`.
      overlays.default = _final: prev: {
        cosmic-ext-vpn-menu = prev.callPackage ./nix/package.nix { };
      };

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            clippy
            rustfmt
            pkg-config
            just
          ];
          buildInputs = runtimeLibs pkgs;
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (runtimeLibs pkgs);
        };
      });

      apps = forAllSystems (pkgs: {
        default = {
          type = "app";
          program = "${self.packages.${pkgs.system}.default}/bin/cosmic-ext-vpn-menu";
        };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);
    };
}
