{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          packages = [
            rust-bin.stable.latest.default
            python314
            uv
            pkg-config
            portaudio
            alsa-lib
          ];

          env = lib.optionalAttrs pkgs.stdenv.isLinux {
            # Python libraries often load native shared objects using dlopen(3).
            # Setting LD_LIBRARY_PATH makes the dynamic library loader aware of libraries without using RPATH for lookup.
            LD_LIBRARY_PATH = lib.makeLibraryPath pkgs.pythonManylinuxPackages.manylinux1;
          };

          shellHook = ''
            unset PYTHONPATH
            uv sync
            . .venv/bin/activate
          '';
        };
      }
    );
}
