# Legacy Nix support (for non-flake setups)
# This file allows `nix-build` to work without flakes
# Usage: nix-build .nix/

(import <nixpkgs> {}).callPackage ./package.nix { }
