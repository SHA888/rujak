# 🦀 Contributing to Rujak

Thank you for considering contributing to **Rujak**! This guide will help you get started.

---

## 📌 How to Contribute

### 🛠 1️⃣ Set Up Your Development Environment

1. **Fork the Repository**  
   - Click the "Fork" button on the [GitHub repo](https://github.com/yourusername/rujak)
   - Clone your fork locally:

     ```bash
     git clone https://github.com/YOUR-USERNAME/rujak.git
     cd rujak
     ```

2. **Set Up the Rust Workspace**

   ```bash
   cargo build --workspace
   ```

3. Run the Backend

   ```
   cd backend
   cargo run
   ```

4. Run the Tests

   ```
   cargo test
   ```

### 🔧 2️⃣ Issue and Feature Requests
    If you find a bug or have a feature request, open an issue in GitHub:
    Open an Issue
    Use clear, concise descriptions.
    Provide steps to reproduce bugs when reporting.

### 📝 3️⃣ Making a Contribution


#### 🔄 1. Create a Feature Branch

```bash
git checkout -b feature/new-feature
```

#### 🛠 2. Make Your Changes
Follow Rust best practices and run:

```bash
cargo fmt  # Format code
cargo clippy  # Lint code
cargo test  # Run tests
```

### ✅ 3. Commit and Push

```bash
git add .
git commit -m "feat: Add new feature"
git push origin feature/new-feature
```

🚀 4. Open a Pull Request

- Go to the repository on GitHub.
- Click "New Pull Request."
- Describe the changes clearly.

## 📜 Code Style & Guidelines
- Rust Format: Use `cargo fmt` before committing.
- Linting: Run `cargo clippy` to ensure code quality.
- Documentation: Use Rust `doc` comments (`///`) for all public functions.

## 🛡 Code of Conduct
We expect all contributors to follow the [Code of Conduct](/docs/CODE_OF_CONDUCT.md) to create a positive, welcoming environment.

## 📬 Contact
GitHub Issues: Report Issues
Discussions: GitHub Discussions

# Happy coding! 🚀


