# 🔮 Git Wizard 🧙

Git Wizard is a command-line tool designed to help beginners find a Git workflow. It provides an intuitive menu-driven interface for common Git operations such as initializing repositories, making changes, branching, merging, and synchronizing with remote repositories.

## Features

- **Initialization and Cloning**: Initialize a new Git repository or clone an existing one.
- **Making Changes**: Add and commit changes to your repository.
- **Branching and Merging**: Create, switch, merge, and delete branches.
- **Synchronization**: Push, pull, and fetch changes from remote repositories.
- **Inspection and Maintenance**: View commit logs, check repository status, and manage stashes.

## Installation

To use Git Wizard, you need to have Rust installed on your system. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/).

Clone the repository and build the project:

```sh
git clone https://github.com/yourusername/git-wizard.git
cd git-wizard
cargo build --release
```

### Docker Image
If you just want to test Git Wizard, you can use the Docker Image here:

## Usage

Run the Git Wizard executable:

```sh
./target/release/git-wizard
```

Git Wizard checks for global environment variables (user.name and user.email). If these are not present on your system, it will prompt you to set them up.

You will then be greeted with a welcome message and the main menu. Use the menu options to navigate through different Git operations.

## Menu Options

### Main Menu

- **[1] Initialization and Cloning**: Initialize a new repository or clone an existing one.
- **[2] Making Changes**: Add and commit changes to your repository.
- **[3] Branching and Merging**: Create, switch, merge, and delete branches.
- **[4] Synchronization**: Push, pull, and fetch changes from remote repositories.
- **[5] Inspection and Maintenance**: View commit logs, check repository status, and manage stashes.
- **[q] Exit**: Exit the Git Wizard.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request on GitHub.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Rust Programming Language](https://www.rust-lang.org/)
- [Git](https://git-scm.com/)
- [Colored](https://github.com/mackwic/colored)
- [Dialoguer](https://github.com/mitsuhiko/dialoguer)
- [Crossterm](https://github.com/crossterm-rs/crossterm)

## Contact

For any questions or suggestions, please send an [email](mailto:aattilio@pm.me).
