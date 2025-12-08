# Contributing to The Prompt of Prompt

Thank you for your interest in contributing to The Prompt of Prompt! 🎉

## How to Contribute

### Reporting Bugs

If you find a bug, please open an issue with:
- A clear description of the problem
- Steps to reproduce
- Expected vs actual behavior
- Your environment (OS, Rust version, etc.)

### Suggesting Features

We welcome feature suggestions! Please open an issue with:
- A clear description of the feature
- Why you think it would be useful
- Any implementation ideas you have

### Pull Requests

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/my-amazing-feature
   ```
3. **Make your changes**
   - Write clean, idiomatic Rust code
   - Add tests if applicable
   - Update documentation as needed
4. **Test your changes**
   ```bash
   cargo test
   cargo build --release
   ```
5. **Commit your changes**
   ```bash
   git commit -m "feat: add my amazing feature"
   ```
6. **Push to your fork**
   ```bash
   git push origin feature/my-amazing-feature
   ```
7. **Open a Pull Request**

## Development Setup

### Prerequisites

- Rust 1.70+
- Google Gemini API Key

### Building

```bash
# Clone the repository
git clone https://github.com/anthropics/the-prompt-of-prompt.git
cd the-prompt-of-prompt

# Build all crates
cargo build

# Run tests
cargo test

# Build release version
cargo build --release
```

### Project Structure

```
the-prompt-of-prompt/
├── crates/
│   ├── gemini-client/      # Gemini API SDK
│   └── prompt-improver/    # CLI tool
├── docs/                   # Documentation
└── Cargo.toml              # Workspace configuration
```

## Code Style

- Follow standard Rust formatting (`cargo fmt`)
- Use `cargo clippy` to check for common issues
- Write documentation for public APIs
- Keep functions small and focused

## Commit Messages

We follow [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` - New features
- `fix:` - Bug fixes
- `docs:` - Documentation changes
- `refactor:` - Code refactoring
- `test:` - Adding/updating tests
- `chore:` - Maintenance tasks

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

Feel free to open an issue if you have any questions!
