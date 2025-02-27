# 🚀 Markdown Server

A fast and lightweight server that renders markdown files with beautiful typography.

## Features

- 📝 GitHub Flavored Markdown support
- 🎨 Beautiful typography with Source Serif font
- 📁 Serves static files alongside markdown
- 📱 Responsive design
- 🔍 Automatic index.md rendering
- ⚡ Built with Rust for maximum performance

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/markdown-server
cd markdown-server

# Build the project
cargo build --release

# Run the server
cargo run --release -- /path/to/your/markdown/files
```

## Usage

### Starting the Server

```bash
# Default port (3000)
cargo run -- /path/to/docs

# Custom port
PORT=8080 cargo run -- /path/to/docs
```

### Directory Structure

docs/
├── index.md # Served at /
├── guide.md # Served at /guide
└── tutorials/
├── index.md # Served at /tutorials/
└── basics.md # Served at /tutorials/basics

### Supported Markdown Features

- Headers (h1-h6)
- Lists (ordered and unordered)
- Code blocks with syntax highlighting
- Tables
- Task lists
- Blockquotes
- Links and images
- Strikethrough
- And more!

## Configuration

The server can be configured using environment variables:

- `PORT`: Server port (default: 3000)

## Development

```bash
# Run tests
cargo test

# Run with hot reloading
cargo watch -x run -- /path/to/docs

# Check formatting
cargo fmt -- --check

# Run linter
cargo clippy
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
