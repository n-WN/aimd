# aimd - AI Streaming Markdown Renderer

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey.svg)](https://github.com/your-username/aimd)


A high-performance streaming Markdown renderer designed for AI command-line tools. Real-time rendering of AI output with syntax highlighting in Markdown format, supporting all mainstream AI CLI tools.

[中文README](README.zh.md) | **English README**

[![asciicast](https://asciinema.org/a/6a0TVOzkpjiXH07AlZLnipxJH.svg)](https://asciinema.org/a/6a0TVOzkpjiXH07AlZLnipxJH)

## ✨ Features

> Temporarily unavailable on Windows, but you can use it in WSL.

- 🚀 **Streaming Rendering** - Real-time AI output rendering without waiting for complete response
- 🎨 **Rich Format Support** - Support for headers, lists, code blocks, bold, italic, and all Markdown elements
- 🔧 **Universal Compatibility** - Support for any AI command-line tool (Gemini, Claude, OpenAI, etc.)
- 📦 **Zero Configuration** - Transparent parameter passing, no need to adapt different tools
- 🐛 **Debug Mode** - Built-in debugging functionality for easy troubleshooting
- 🔄 **Pipe Friendly** - Perfect support for Unix pipe operations
- 🌐 **Multi-language** - Automatic language detection (Chinese/English)

## 🎯 Supported Markdown Formats

| Format | Syntax | Rendered Effect |
|--------|--------|-----------------|
| Headers | `# Header` | 🔵 Colored headers |
| Bold | `**bold**` | **Yellow bold** |
| Italic | `*italic*` | *Italic text* |
| Bold Italic | `***bold italic***` | ***Yellow bold italic*** |
| Inline Code | `code` | 🟫 Orange-red code |
| Code Block | \`\`\`language<br/>code<br/>\`\`\` | 📦 Bordered code block |
| Lists | `• item` | 🟣 Purple lists |
| Quotes | `> quote` | 🟡 Yellow border |

## 🚀 Quick Start

### Installation

Ensure you have Rust 1.70+ installed:

```bash
# Clone repository
git clone https://github.com/your-username/aimd.git
cd aimd

# Build
cargo build --release

# Install to system
cargo install --path .
```

### Basic Usage

#### 1. Show Help
```bash
aimd
```

#### 2. Pipe Input (Recommended)
```bash
echo "Explain what Rust programming language is" | aimd
```

#### 3. Explicit Command
```bash
aimd -- gemini --model gemini-2.5-flash -p "Introduce Markdown syntax"
```

## 📖 Usage Guide

### Pipe Mode

Pipe mode is the most concise way to use, input content will be automatically passed as prompt to default Gemini:

```bash
# Simple question
echo "What is machine learning?" | aimd

# File content as input
cat question.txt | aimd

# Multi-line input
cat << EOF | aimd
Please introduce in Markdown format:
1. Python features
2. Common libraries
3. Application scenarios
EOF
```

### Explicit Command Mode

Support for any AI CLI tool, just specify the complete command after `--`:

```bash
# Gemini
aimd -- gemini --model gemini-2.5-flash -p "Introduce Rust"

# Claude
aimd -- claude --model sonnet -p "Explain async/await"

# OpenAI
aimd -- openai api chat.completions.create -m gpt-4 --messages '[{"role":"user","content":"Hello"}]'

# Custom tools
aimd -- my-ai-tool --custom-param value "prompt"
```

### Debug Mode

Use `--debug` to view detailed execution information:

```bash
# Debug pipe input
echo "Test input" | aimd --debug

# Debug explicit command
aimd --debug -- gemini --help
```

## 🔧 Configuration Options

| Parameter | Description | Example |
|-----------|-------------|---------|
| `--debug` | Enable debug mode, show execution details | `aimd --debug` |
| `--help` | Show help information | `aimd --help` |
| `--` | Parameter separator, subsequent parameters passed to AI tool | `-- gemini -p "hello"` |

## 📊 Performance Features

- **Low Latency**: Streaming processing, first byte response time < 10ms
- **Memory Efficient**: Line buffer processing, constant memory usage
- **Cross-platform**: Support macOS, Linux, Windows
- **Zero Configuration**: Works out of the box, no configuration files needed

## 🛠️ Technical Architecture

```
Input Source → PTY Process → Stream Parsing → Markdown Rendering → Terminal Output
     ↓            ↓             ↓                ↓                    ↓
  Pipe/Args    AI Tool      Line Processing   ANSI Coloring      Real-time Display
```

### Core Components

- **PTY Management**: Using `pty-process` to manage pseudo terminals
- **Stream Parsing**: Custom state machine for Markdown parsing
- **ANSI Rendering**: Native ANSI escape sequences for coloring
- **Parameter Passthrough**: Zero-loss parameter passing mechanism

## 🌐 Language Support

The program automatically detects system language based on environment variables:

- **Chinese**: When `LANG`, `LC_ALL`, or `LC_MESSAGES` starts with `zh`
- **English**: Default for all other cases

Environment variables checked (in order):
1. `LANG`
2. `LC_ALL` 
3. `LC_MESSAGES`

## 🤝 AI Tool Compatibility

| Tool | Status | Tested Version | Notes |
|------|--------|----------------|-------|
| Gemini CLI | ✅ Full Support | latest | Google official CLI |
| Claude CLI | ✅ Full Support | latest | Anthropic official CLI |
| OpenAI CLI | ✅ Full Support | latest | OpenAI official CLI |
| Ollama | ✅ Full Support | v0.1.0+ | Local AI models |
| Custom Tools | ✅ Full Support | - | Any command that outputs text |

## 🎨 Rendering Examples

### Input
```markdown
# AI Tool Comparison

## Main Features

- **Gemini**: Google's multimodal AI
- **Claude**: Anthropic's conversational AI  
- **GPT**: OpenAI's generative AI

### Code Example

```python
import openai
client = openai.OpenAI()
response = client.chat.completions.create(
    model="gpt-4",
    messages=[{"role": "user", "content": "Hello!"}]
)
print(response.choices[0].message.content)
```

> Choose the right AI tool based on specific needs
```

### Rendered Output
![Rendering Example](docs/render-example.png)

## 📝 Development

### Build Requirements

- Rust 1.70.0+
- Cargo

### Development Environment

```bash
# Clone and enter directory
git clone https://github.com/your-username/aimd.git
cd aimd

# Run tests
cargo test

# Development mode run
cargo run -- --debug -- echo "# Test\n**Bold text**"

# Release build
cargo build --release
```

### Project Structure

```
aimd/
├── src/
│   └── main.rs              # Main program logic
├── Cargo.toml              # Dependency configuration
├── README.md              # Chinese documentation
├── README.en.md           # English documentation
└── docs/                  # Documentation resources
```

## 🐛 Troubleshooting

### Common Issues

1. **AI tool not found**
   ```bash
   # Ensure AI tool is installed and in PATH
   which gemini
   ```

2. **Permission issues**
   ```bash
   # Ensure executable permissions
   chmod +x target/release/aimd
   ```

3. **Encoding issues**
   ```bash
   # Set correct terminal encoding
   export LANG=en_US.UTF-8
   ```

### Debug Steps

1. Use `--debug` to view detailed information
2. Check if AI tool works normally
3. Verify parameter passing is correct

## 🤝 Contributing

Contributions are welcome! Please follow this workflow:

1. Fork this repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

### Code Standards

- Use `cargo fmt` to format code
- Use `cargo clippy` for code quality checks
- Add necessary test cases

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [pty-process](https://crates.io/crates/pty-process) - PTY management
- [Rust Community](https://www.rust-lang.org/community) - Excellent ecosystem
- All contributors and users for their support

## 📞 Contact

- Project Homepage: [GitHub](https://github.com/your-username/aimd)
- Issue Reports: [Issues](https://github.com/your-username/aimd/issues)
- Feature Requests: [Discussions](https://github.com/your-username/aimd/discussions)

---

⭐ If this project helps you, please give us a star! 