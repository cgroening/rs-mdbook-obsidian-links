# mdbook-obsidian-links

A preprocessor for [mdBook](https://github.com/rust-lang/mdBook) that converts Obsidian-style wiki links to standard Markdown links.

## Features

Converts Obsidian-style links in all variants to mdBook-compatible Markdown links:

| Obsidian Format | Converted to |
|----------------|--------------|
| `[[chapter#Section Name\|Display Text]]` | `[Display Text](chapter.md#section-name)` |
| `[[chapter#Section Name]]` | `[chapter](chapter.md#section-name)` |
| `[[chapter\|Display Text]]` | `[Display Text](chapter.md)` |
| `[[chapter]]` | `[chapter](chapter.md)` |

**Key transformations:**

- Converts headings to lowercase
- Replaces spaces and underscores with hyphens in anchors
- Preserves custom display text
- Adds `.md` extension automatically

## Installation

### Option 1: Build and use locally

1. Clone the repository:

```zsh
git clone https://github.com/cgroening/rs-mdbook-obsidian-links.git
cd mdbook-obsidian-links
```

2. Build the preprocessor:

```zsh
cargo build --release
```

3. The binary will be in `target/release/mdbook-obsidian-links`

4. Add to your `book.toml`:

```toml
[preprocessor.obsidian-links]
command = "/absolute/path/to/mdbook-obsidian-links/target/release/mdbook-obsidian-links"
```

### Option 2: Install globally

1. Clone and install:

```zsh
git clone https://github.com/yourusername/mdbook-obsidian-links.git
cd mdbook-obsidian-links
cargo install --path .
```

2. Add to your `book.toml`:

```toml
[preprocessor.obsidian-links]
```

The preprocessor will be automatically found in your `PATH.`

## Usage

Once configured in `book.toml`, the preprocessor runs automatically when you build your book:

```zsh
mdbook build
```

## Examples

### Before (Obsidian format)

```markdown
# My Chapter

See the [[introduction#Getting Started|intro guide]] for basics.

For advanced topics, check [[advanced]] or [[api#Methods]].

Related: [[concepts|Core Concepts]].
```

### After (mdBook format)

```markdown
# My Chapter

See the [intro guide](introduction.md#getting-started) for basics.

For advanced topics, check [advanced](advanced.md) or [api](api.md#methods).

Related: [Core Concepts](concepts.md).
```

## Testing

Run the test suite:

```zsh
cargo test
```

Expected output:

```
running 4 tests
test tests::test_convert_anchor ... ok
test tests::test_convert_obsidian_links_all_variants ... ok
test tests::test_multiple_links ... ok
test tests::test_no_conversion_needed ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured
```

### Manual testing

Create a test mdBook project:

```zsh
mdbook init test-book
cd test-book
```

Add the preprocessor to `book.toml`:

```toml
[preprocessor.obsidian-links]
command = "/path/to/mdbook-obsidian-links"
```

Add Obsidian links to `src/SUMMARY.md` or any chapter:

```markdown
# Summary

- [Chapter 1](chapter_1.md)
  - [[chapter_1#Section A|Go to Section A]]
  - [[chapter_2|Next Chapter]]
```

Build and verify:

```zsh
mdbook build
# Check the generated HTML in book/
```

## How it works

The preprocessor:

1. Receives the book content as JSON from mdBook via stdin
2. Recursively processes all chapters
3. Uses regex to find and replace Obsidian-style links
4. Returns the modified book as JSON to stdout
5. mdBook continues with the standard rendering process

## Requirements

- Rust 1.70 or later
- mdBook 0.4.0 or later

## Compatibility

Tested with mdBook versions 0.4.x. The preprocessor supports all mdBook renderers (HTML, PDF, etc.).

## Troubleshooting

### "Missing 'book' in input" error

This usually means the preprocessor is receiving unexpected input. Ensure you're using mdBook 0.4.0 or later.

### Links not converting

- Verify the preprocessor is listed when running `mdbook build -v`
- Check that links match the exact Obsidian format: `[[file#section|text]]`
- Ensure the preprocessor has execute permissions: `chmod +x /path/to/mdbook-obsidian-links`

### Version mismatch warning

If you see a version mismatch warning, rebuild the preprocessor against your mdBook version:

```zsh
cargo clean
cargo build --release
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development setup

```zsh
git clone https://github.com/yourusername/mdbook-obsidian-links.git
cd mdbook-obsidian-links
cargo build
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [mdBook](https://github.com/rust-lang/mdBook) - The awesome book builder this preprocessor extends
- Inspired by Obsidian's wiki-link syntax
