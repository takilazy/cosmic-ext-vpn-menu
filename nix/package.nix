# SPDX-License-Identifier: MPL-2.0
# The single source of truth for the package derivation, shared by flake.nix
# (via callPackage) and default.nix (the non-flakes entry point).
{
  lib,
  rustPlatform,
  pkg-config,
  wayland,
  libxkbcommon,
  vulkan-loader,
  libGL,
  fontconfig,
  freetype,
  expat,
}:
let
  appId = "com.github.takilazy.CosmicExtVpnMenu";

  # Libraries libcosmic (winit/wayland/wgpu) dlopens or links at runtime.
  runtimeLibs = [
    wayland
    libxkbcommon
    vulkan-loader
    libGL
    fontconfig
    freetype
    expat
  ];
in
rustPlatform.buildRustPackage {
  pname = "cosmic-ext-vpn-menu";
  version = "0.1.0";

  # Keep target/, .git, and Nix build symlinks out of the source copy.
  src = lib.cleanSourceWith {
    src = ../.;
    filter =
      path: _type:
      let
        base = baseNameOf path;
      in
      base != "target" && base != ".git" && base != "result";
  };

  # libcosmic and friends come from git; let Nix fetch them per Cargo.lock's
  # recorded revisions (no per-dependency hashes to maintain).
  cargoLock = {
    lockFile = ../Cargo.lock;
    allowBuiltinFetchGit = true;
  };

  # NB: do NOT put `just` here. The nixpkgs `just` package ships setup-hooks
  # that hijack build/check/install to `just build|test|install`, and the
  # justfile's `install` targets /usr — which fails in the sandbox. We build
  # with cargo (buildRustPackage's default phases) and install resources in
  # postInstall with coreutils `install`.
  nativeBuildInputs = [
    pkg-config
  ];
  buildInputs = runtimeLibs;

  # Install the desktop entry, metainfo, and icon alongside the binary.
  postInstall = ''
    install -Dm0644 resources/app.desktop \
      "$out/share/applications/${appId}.desktop"
    install -Dm0644 resources/app.metainfo.xml \
      "$out/share/metainfo/${appId}.metainfo.xml"
    install -Dm0644 resources/icon.svg \
      "$out/share/icons/hicolor/scalable/apps/${appId}.svg"
  '';

  # Ensure the dlopen'd wayland/GL/Vulkan libraries resolve at runtime.
  postFixup = ''
    patchelf --add-rpath "${lib.makeLibraryPath runtimeLibs}" \
      "$out/bin/cosmic-ext-vpn-menu"
  '';

  meta = {
    description = "VPN management applet for the COSMIC desktop";
    homepage = "https://github.com/takilazy/cosmic-ext-vpn-menu";
    license = lib.licenses.mpl20;
    mainProgram = "cosmic-ext-vpn-menu";
    platforms = lib.platforms.linux;
  };
}
