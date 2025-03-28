<div align="center">
  <h1>🚀 Markdown Server</h1>
  <p>A fast and lightweight server that serves markdown files as HTML pages with beautiful typography.</p>
</div>

## Example

<div align="center">
<img src="docs/screenshot.png" alt="Markdown Server Screenshot" width="500" />
</div>

## Features

- 📝 GitHub Flavored Markdown support
- 🎨 Beautiful typography with Source Serif font
- 📁 Serves static files alongside markdown
- 📱 Responsive design
- 🔍 Automatic index.md rendering
- 🛠️ Customizable with YAML configuration
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

# With configuration file
cargo run -- serve --directory /path/to/docs --config /path/to/config.yaml
```

### Exporting Static HTML

```bash
# Export markdown to HTML
cargo run -- export --input-dir /path/to/docs --output-dir /path/to/html

# With custom template and configuration
cargo run -- export --input-dir /path/to/docs --output-dir /path/to/html --template /path/to/template.html --config /path/to/config.yaml
```

### Directory Structure

```
docs/
├── index.md # Served at /
├── guide.md # Served at /guide
└── tutorials/
    ├── index.md # Served at /tutorials/
    └── basics.md # Served at /tutorials/basics
```

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

### Environment Variables

- `PORT`: Server port (default: 3000)

### YAML Configuration File

You can customize the appearance and content of your markdown site using a YAML configuration file. Here's an example:

```yaml
# Custom CSS to be injected into the HTML page
custom_css: |
  body {
    max-width: 85ch;  /* Wider content area */
  }

  h1, h2, h3 {
    color: #1e5285;  /* Custom heading colors */
  }

# Custom HTML content for the page header
header: |
  <div style="text-align: center; padding: 20px;">
    <img src="/logo.png" alt="Logo" height="60">
  </div>

# Custom HTML content for the page footer
footer: |
  <div style="text-align: center; margin-top: 30px;">
    <p>© 2025 Your Organization</p>
  </div>

# Navigation links to be displayed in the header
navigation:
  - text: Home
    url: /
  - text: Documentation
    url: /docs
  - text: GitHub
    url: https://github.com/intellicode/mdserve
```

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
