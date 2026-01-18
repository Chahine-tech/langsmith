# Nix Configuration Files

This directory contains supplementary Nix configurations for advanced use cases.

## Files

- `shell.nix` - (Optional) Legacy shell.nix for compatibility
- `default.nix` - (Optional) Legacy default.nix for compatibility

## Using flake.nix (Recommended)

Use the modern flake.nix at the root instead:

```bash
nix develop              # Enter dev shell
nix build                # Build package
nix build .#docker       # Build Docker image
```

See [docs/NIX.md](../docs/NIX.md) for full documentation.
