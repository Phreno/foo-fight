# foo-fight

Speed typing training for command-line commands (CLI app built with Rust, ratatui, and crossterm).

## Description

Foo Fight is a terminal-based speed typing trainer inspired by ShortcutFoo. It helps you practice and memorize command-line commands through interactive training sessions.

## Features

- 📚 Multiple dictionaries (Git, Jujutsu, PowerShell/Pester)
- 🎯 Interactive training sessions
- ✅ Command validation with alias support
- 📊 Real-time statistics (accuracy tracking)
- 🖥️ Cross-platform (Windows, Linux, macOS)
- 🎨 Beautiful terminal UI

## Installation

### Prerequisites

- Rust 1.70 or later

### Building from source

```bash
git clone https://github.com/Phreno/foo-fight.git
cd foo-fight
cargo build --release
```

The binary will be available at `target/release/foo-fight`.

## Usage

Run the application:

```bash
cargo run
```

Or run the compiled binary:

```bash
./target/release/foo-fight
```

### Controls

#### Dictionary Selection Screen
- `↑`/`↓` - Navigate between dictionaries
- `Enter` - Select dictionary and start training
- `q` - Quit application

#### Training Screen
- Type your answer and press `Enter` to submit
- `Esc` - Return to dictionary selection

#### Results Screen
- `Enter` - Return to dictionary selection
- `q` - Quit application

## Dictionary Format

Dictionaries are TOML files stored in the `dictionaries/` folder. Each dictionary has the following format:

```toml
name = "Dictionary Name"
description = "Description of the dictionary"

[[items]]
question = "What command does X?"
command = "expected-command"
aliases = ["alternative1", "alternative2"]
```

### Creating Your Own Dictionary

1. Create a new `.toml` file in the `dictionaries/` folder
2. Follow the format shown above
3. Add your questions, commands, and aliases
4. Restart the application to see your new dictionary

## Example Dictionaries

The project includes three example dictionaries:

- **Git Commands** - Common Git version control commands
- **Jujutsu Commands** - Jujutsu VCS commands
- **PowerShell/Pester** - PowerShell and Pester testing commands

## Development

### Running in development mode

```bash
cargo run
```

### Building for release

```bash
cargo build --release
```

## License

MIT

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
 
