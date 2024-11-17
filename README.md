# Konfigura

A command-line interface (CLI) tool to manage Linux kernel configuration files.

## Features

- Initialize a data store for Linux kernel configuration files
- Add and manage new configuration files
- Update existing configuration files
- Remove configuration files
- List all available configuration files
- Display the contents of specific configuration files
- Utilize Git as a backend for version control and flexibility

## Installation

### Prerequisites

Ensure you have Rust and Cargo installed on your system. If not, follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).

### Option 1: Install via Cargo

```bash
cargo install --path .
```

### Option 2: Build from Source

```bash
git clone https://github.com/yourusername/konfigura.git
cd konfigura
cargo build --release
```

The compiled binary will be available at `target/release/konfigura`.

## Usage

### Initializing the Data Store

Before using Konfigura, initialize the data store:

```bash
konfigura init
```

This command creates a Git repository in the `$XDG_DATA_HOME/konfigura/repo/` directory, serving as the backend storage for your configuration files.

### Basic Commands

- Add a new configuration: `konfigura add <file_path>`
- Update an existing configuration: `konfigura update <config_name>`
- Remove a configuration: `konfigura remove <config_name>`
- List all configurations: `konfigura list`
- Show a specific configuration: `konfigura show <config_name>`

For a complete list of commands and options, use:

```bash
konfigura -h
```

## Advanced Usage

Konfigura leverages Git for version control, allowing users to:

- Track changes to configurations over time
- Revert to previous versions if needed
- Use standard Git commands for advanced management

## Contributing

Contributions are welcome!

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

If you encounter any issues or have questions, please file an issue on the [GitHub issue tracker](https://github.com/yourusername/konfigura/issues).
