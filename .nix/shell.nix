# Legacy shell.nix for non-flake Nix users
# Usage: nix-shell .nix/shell.nix

let
  nixpkgs = import <nixpkgs> { };
in

nixpkgs.mkShell {
  buildInputs = with nixpkgs; [
    rustup
    pkg-config
    openssl
    cargo-watch
    cargo-edit
    cargo-outdated
  ];

  shellHook = ''
    echo "🧙 Langsmith dev environment (legacy nix-shell)"
    echo "Run: cargo build, cargo test, cargo run -- --help"
    echo ""
    echo "⚠️  Note: Flakes are recommended. Use 'nix develop' instead."
  '';
}
