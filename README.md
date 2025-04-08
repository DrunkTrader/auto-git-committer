# Auto Git Committer

A Rust-based tool that automates Git operations including repository initialization, commits, and pushing changes to remote repositories. Available as both a command-line interface (CLI) and graphical user interface (GUI).

## Features

- **Repository Management**
  - Automatically clones repositories from remote URLs
  - Initializes new Git repositories with proper setup
  - Configures remote origin and main branch
  - Creates initial README.md and .gitignore files

- **Automated Commits**
  - Performs automated file operations (create, rename, delete)
  - Generates sequential commits with unique messages
  - Handles Git add, commit, and push operations

- **Error Handling**
  - Comprehensive error handling for all Git operations
  - Detailed error messages for troubleshooting
  - Graceful handling of repository states

- **Multiple Interfaces**
  - Command-line interface for scripting and automation
  - Graphical user interface for easy interaction

## Requirements

- Rust and Cargo (Rust's package manager)
- Git installed and configured on your system
- Basic knowledge of Git operations

## Installation

### 1. Clone the repository:
   ```bash
   git clone https://github.com/DrunkTrader/auto-git-committer.git
   cd auto-git-committer
   ```

### 2. Build the project:
   ```bash
   cargo build --release
   ```

### 3. Install the binary (Optional):
   ```bash
   cargo install --path .
   ```

## Usage

### Command Line Interface

```bash
# Basic usage CLI command
# Replace <repository_name> and <repository_url> with actual values & <commit_count> with the number of commits you want to make [(optional) default is 5]
cargo run cli -r "<repository_name>" -u "<repository_url>" -c <commit_count>

# Example
cargo run cli -r "my-project" -u "https://github.com/user/my-project.git" -c 5
```

### Graphical User Interface

```bash
# Launch the GUI
cargo run gui
```


## Project Structure

The project is organized into several modules:
- `main.rs`: Entry point and application setup
- `logic.rs`: Core business logic for Git operations
- `cli.rs`: Command-line interface implementation
- `gui.rs`: Graphical user interface implementation

## How It Works

1. **Repository Setup**
   - If the repository doesn't exist, it will be cloned from the provided URL
   - If the directory is empty, a new Git repository will be initialized
   - Initial files (README.md, .gitignore) are created automatically

2. **Commit Automation**
   - The program performs file operations to generate changes
   - Changes are automatically staged and committed
   - Commits are pushed to the remote repository

3. **Error Handling**
   - Each operation includes error checking and reporting
   - Failed operations are logged with detailed error messages
   - The program attempts to recover from common errors

## Command Line Options

```
USAGE:
    auto-git-committer [OPTIONS]

OPTIONS:
    -g, --gui       Launch with graphical user interface
    -h, --help      Print help information
    -v, --version   Print version information
```
## License

This project is licensed under the MIT License, which permits reuse, modification, and distribution with attribution. See the [LICENSE](LICENSE) file for details.
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

Before contributing, please check the existing issues or discussions to ensure your contribution doesn't duplicate existing efforts.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
## Author

- Neeraj Kumar (neerajrajputa786@gmail.com)

## Repository

- GitHub: [https://github.com/DrunkTrader/auto-git-committer](https://github.com/DrunkTrader/auto-git-committer)
