# dependencies for compiling a `crossd` program.
# 
# for now, this is for the Vulkan Wgpu backend. OpenGL support
# would require at least `pkgs.mesa` (likely more).

{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
  };

  outputs = inputs @ {
    nixpkgs,
    flake-parts,
    ...
  }: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [
      "x86_64-linux"
      "aarch64-linux"
      "aarch64-darwin"
      "x86_64-darwin"
    ];
    perSystem = { config, self', inputs', pkgs, system, ... }: let
      buildInputs = with pkgs; [
        vulkan-loader
      ];
      nativeBuildInputs = with pkgs; [
        pkg-config
      ];
      runtimeDependencies = with pkgs; [
        vulkan-loader

        # not strictly required, but required by winit
        wayland
        xkbcommon
        libX11
        libXcursor
        libXi
        libXrandr
      ];

      LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath buildInputs;
    in {
      # `nix develop`
      devShells.default = pkgs.mkShell {
        inherit buildInputs nativeBuildInputs LD_LIBRARY_PATH;
      };

      # TODO: tests
    };
  };
}
