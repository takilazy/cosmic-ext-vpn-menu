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

      appId = "com.github.takilazy.CosmicExtVpnMenu";

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
        cosmic-ext-vpn-menu = pkgs.rustPlatform.buildRustPackage {
          pname = "cosmic-ext-vpn-menu";
          version = "0.1.0";
          src = ./.;

          # libcosmic and friends come from git; let Nix fetch them per Cargo.lock's
          # recorded revisions (no per-dependency hashes to maintain).
          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = true;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
            just
          ];
          buildInputs = runtimeLibs pkgs;

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
            patchelf --add-rpath "${pkgs.lib.makeLibraryPath (runtimeLibs pkgs)}" \
              "$out/bin/cosmic-ext-vpn-menu"
          '';

          meta = with pkgs.lib; {
            description = "VPN management applet for the COSMIC desktop";
            homepage = "https://github.com/takilazy/cosmic-ext-vpn-menu";
            license = licenses.mpl20;
            mainProgram = "cosmic-ext-vpn-menu";
            platforms = platforms.linux;
          };
        };

        default = cosmic-ext-vpn-menu;
      });

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
