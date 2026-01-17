# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Langsmith** is a framework-agnostic CLI tool for automatic i18n (internationalization) string extraction, translation, and code replacement. It automates the tedious process of setting up internationalization in JavaScript/TypeScript projects.

**Current Status**: Phase 2 complete (extract, translate, replace, merge commands implemented). Phase 3 is polish/distribution.

## Architecture

Langsmith follows **Clean Architecture** with strict separation of concerns:

```
src/
├── domain/              # Core business logic (0 dependencies)
│   ├── models.rs        # TranslationKey, FileType, ReplacementStrategy, etc.
│   └── ports.rs         # Traits: StringExtractor, Translator, CodeReplacer, etc.
│
├── application/         # Use cases (orchestration layer)
│   ├── extract_strings.rs       # Phase 1: Extract strings
│   ├── translate_keys.rs        # Phase 2: Translate via API
│   ├── replace_strings.rs       # Phase 2: Replace in code with t("key")
│   └── merge_i18n.rs            # Phase 2: Merge .i18n.* files
│
├── infrastructure/      # Implementations
│   ├── string_extractor/        # Regex-based JS/TS string extraction
│   ├── translators/             # DeepL, OpenAI API clients
│   ├── code_replacer/           # Regex-based code replacement
│   ├── file_system.rs           # File I/O operations
│   └── config.rs                # API key and config management
│
└── cli/                 # User interface
    ├── commands/        # extract.rs, translate.rs, replace.rs, merge.rs
    ├── mod.rs           # Command enum and router
    └── presenter.rs     # Output formatting
```

### Key Design Patterns

1. **Port-Adapter Pattern**: Traits in `domain/ports.rs` (StringExtractor, Translator, CodeReplacer, ImportManager) enable dependency injection and swappable implementations
2. **Async-first**: All I/O is async using tokio runtime
3. **Error Handling**: Uses `anyhow::Result` for convenience, context-rich errors
4. **Byte Position Tracking**: `TranslationKeyWithPosition` tracks exact file positions for precise string replacement

## Build & Run

```bash
# Build debug binary
cargo build

# Build release binary (optimized)
cargo build --release

# Run CLI
./target/debug/langsmith --help

# Run specific command
./target/debug/langsmith extract ./src --output ./i18n
./target/debug/langsmith translate ./i18n/fr.json --to en,es --api deepl
./target/debug/langsmith replace ./src --translations ./i18n/fr.json --dry-run
./target/debug/langsmith merge ./src --confirm
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_format_key

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test '*'
```

## Complete Workflow Example

```bash
# 1. Extract strings from codebase
./target/debug/langsmith extract ./examples/sample-app --output ./examples/sample-app/i18n

# 2. Translate using DeepL API (requires DEEPL_API_KEY env var)
export DEEPL_API_KEY="your-key:fx"
./target/debug/langsmith translate ./examples/sample-app/i18n/fr.json --to en,de --api deepl

# 3. Replace strings in code (creates .i18n.* files by default)
./target/debug/langsmith replace ./examples/sample-app --translations ./examples/sample-app/i18n/fr.json --dry-run

# 4. If happy with preview, actually generate .i18n.* files
./target/debug/langsmith replace ./examples/sample-app --translations ./examples/sample-app/i18n/fr.json

# 5. Review changes in git
git diff

# 6. Merge .i18n.* files back to originals (with confirmation)
./target/debug/langsmith merge ./examples/sample-app --confirm
```

## Command Reference

### Extract
```bash
langsmith extract <PATH> [--output <DIR>] [--lang <LANG>]
```
Scans directory for `.js`, `.jsx`, `.ts`, `.tsx`, `.vue`, `.html` files and extracts translatable strings. Creates `<output>/<lang>.json` with extracted strings as keys and values.

### Translate
```bash
langsmith translate <FILE> --to <LANGS> --api <PROVIDER> [--api-key <KEY>]
```
Translates JSON translation file using specified API (deepl or openai). Generates language files for each target language. Requires API key via CLI flag or environment variable (DEEPL_API_KEY or OPENAI_API_KEY).

### Replace
```bash
langsmith replace <PATH> --translations <FILE> [--strategy <STRATEGY>] [--dry-run] [--in-place]
```
Replaces hardcoded strings in source code with translation function calls. Non-destructive by default (creates `.i18n.*` files). Use `--in-place` to modify originals or `--dry-run` to preview changes. Strategies: `react-i18n`, `vue-i18n`, `generic`.

### Merge
```bash
langsmith merge <PATH> [--confirm]
```
Finds `.i18n.*` files and merges them back to originals (renames `.i18n.tsx` → `.tsx`). Preview mode by default; use `--confirm` to actually merge.

## Important Files & Concepts

### String Extraction: `js_extractor.rs`
- **Current approach**: Regex-based (Phase 1 MVP)
- **Position tracking**: `extract_with_positions()` returns `TranslationKeyWithPosition` with byte offsets
- **Filtering logic**: `should_extract()` excludes package names, imports, paths, short strings
- **Key generation**: `format_key()` converts "Hello World" → "hello_world"

### Code Replacement: `regex_replacer.rs` + `import_manager.rs`
- **Back-to-front replacement**: Sort by `start_byte DESC` to preserve offsets during replacement
- **JSX detection**: `detect_jsx_context()` checks bracket balance to wrap in `{}`
- **Import deduplication**: `SimpleImportManager` checks if import exists before adding

### Translation APIs: `deepl.rs` + `openai.rs`
- **DeepL**: Uses `https://api-free.deepl.com/v2/translate` endpoint (v1 is deprecated)
- **Language normalization**: "en" → "EN-US" for DeepL, "English" for OpenAI
- **Rate limiting**: 100ms delay between requests to avoid throttling

### API Key Management: `config.rs`
- **Priority order**: CLI flag > Environment variable > Error
- **Environment variables**: `DEEPL_API_KEY` or `OPENAI_API_KEY`

## Known Limitations (MVP)

1. **Extraction**: Regex-based (Phase 1). Full AST parsing planned for future
2. **Template literals**: Partial support for `` `text` ``
3. **Complex JSX**: Deeply nested or conditional JSX may need manual review
4. **Single quote strings**: Currently only double quotes fully supported
5. **Formatting**: Code replacement may alter whitespace (run prettier after)

## Common Development Tasks

### Add a new translation API (e.g., Google Translate)

1. Create `src/infrastructure/translators/google.rs` implementing `Translator` trait
2. Add `GoogleTranslator` variant to config module
3. Update `ConfigManager::get_api_config()` to handle new provider
4. Add test in `translate.rs` CLI command

### Enhance string extraction

1. Modify `src/infrastructure/string_extractor/js_extractor.rs`
2. Update filtering logic in `should_extract()`
3. Add new regex patterns for edge cases
4. Test on `examples/sample-app`

### Add framework-specific replacement strategy

1. Add new variant to `ReplacementStrategy` enum in `src/domain/models.rs`
2. Update `import_statement()` and `translate_call()` methods
3. Create corresponding test in CLI replace command

## Dependencies & Versions

Key crates (see `Cargo.toml`):
- **clap 4.5**: CLI argument parsing
- **tokio 1.49**: Async runtime
- **serde/serde_json 1.0**: JSON serialization
- **regex 1.12**: String pattern matching
- **reqwest 0.11**: HTTP client for APIs
- **async-trait 0.1**: Async trait support
- **tracing/tracing-subscriber 0.3**: Logging
- **anyhow 1.0**: Error handling

## Testing Strategy

- Unit tests in each module (use `#[cfg(test)]` blocks)
- Integration tests with `examples/sample-app` as fixture
- CLI tests using `assert_cmd` + `predicates` crates
- Always test edge cases: empty inputs, special characters, large files

## Debugging Tips

- **Enable debug logging**: `RUST_LOG=debug cargo run -- <command>`
- **Inspect API responses**: Check `DeepL API response status` in logs
- **File I/O issues**: Verify paths exist with `ls -la`
- **API key problems**: Check env var with `echo $DEEPL_API_KEY`
- **Byte position bugs**: Print start_byte/end_byte values and compare with actual file

## Phase Roadmap

- **Phase 1** ✅ DONE: String extraction
- **Phase 2** ✅ DONE: Translation + Code replacement + Merge workflow
- **Phase 3** ⏳ TODO: Distribution (crates.io, GitHub releases, docs)

## Git Workflow

- Main branch: `main` (always deployable)
- Feature branches: Not used in this project
- Commits: Use conventional commits (`feat:`, `fix:`, `docs:`)
- Co-authoring: Add `Co-Authored-By: Claude Haiku 4.5 <noreply@anthropic.com>` to commits
