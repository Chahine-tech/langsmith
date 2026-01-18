{
  description = "Langsmith - Automatic i18n extraction and translation CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    pre-commit-hooks.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, pre-commit-hooks }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
        };

        version = "0.1.0";

      in
      {
        # Development shell with all tools
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
            cargo-watch
            cargo-edit
            cargo-outdated
            cargo-deny
            cargo-audit
            pkg-config
            openssl
            pre-commit
            git
          ];

          shellHook = ''
            echo "🧙 Langsmith dev environment loaded (Nix)"
            echo "Run: cargo build, cargo test, cargo run -- --help"
          '';
        };

        # Production build (for current system)
        packages.default = pkgs.rustPlatform.buildRustPackage {
          name = "langsmith";
          inherit version;
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;

          buildInputs = with pkgs; [
            pkg-config
            openssl
          ];

          meta = with pkgs.lib; {
            description = "Automatic i18n extraction and translation CLI";
            homepage = "https://github.com/chahinebenlahcen/langsmith";
            license = licenses.mit;
            maintainers = [];
            mainProgram = "langsmith";
          };
        };

        # Docker image
        packages.docker = pkgs.dockerTools.buildLayeredImage {
          name = "langsmith";
          tag = version;
          contents = [ self.packages.${system}.default pkgs.cacert ];
          config = {
            Cmd = [ "${self.packages.${system}.default}/bin/langsmith" ];
            Env = [ "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt" ];
          };
        };

        # CLI app
        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/langsmith";
        };

        # Quality checks (disabled in flake check, available in nix develop)
        checks = { };
      }
    );
}
