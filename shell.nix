# SPDX-License-Identifier: MPL-2.0
#
# Dev shell for building `cosmic-ext-vpn-menu` on NixOS.
#   nix-shell            # enter the shell
#   just build-release   # build
#   just run             # build & run
#
# Verified against nixpkgs 26.05: `cargo check` of the libcosmic applet template
# builds clean with exactly these inputs.
{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell rec {
  nativeBuildInputs = with pkgs; [
    cargo
    rustc
    rust-analyzer
    clippy
    rustfmt
    pkg-config
    just
  ];

  # Runtime/link libraries libcosmic + iced (winit/wayland/wgpu) need.
  buildInputs = with pkgs; [
    wayland
    libxkbcommon
    vulkan-loader
    libGL
    fontconfig
    freetype
    expat
  ];

  # Wayland/GL/Vulkan are dlopen'd at runtime, so they must be on the loader path.
  LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
}
