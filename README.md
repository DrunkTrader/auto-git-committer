# Auto Git Committer

A Rust-based command-line tool that automates Git operations including repository initialization, commits, and pushing changes to remote repositories.

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

## Requirements

- Rust and Cargo (Rust's package manager)
- Git installed and configured on your system
- Basic knowledge of Git operations

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/DrunkTrader/auto-git-committer.git
   cd auto-git-committer
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the program:
   ```bash
   cargo run
   ```

## Usage

1. Run the program:
   ```bash
   cargo run
   ```

2. Follow the interactive prompts:
   - Enter the repository name
   - Provide the Git repository URL (optional)
   - The program will handle the rest automatically

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Author

- Neeraj Kumar (neerajrajputa786@gmail.com)

## Repository

- GitHub: [https://github.com/DrunkTrader/auto-git-committer](https://github.com/DrunkTrader/auto-git-committer)

## Contribute to the project

- Fork the repository
- Make your changes
- Create a Pull Request
- Wait for the PR to be merged
- Celebrate!
