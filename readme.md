# Cuery Command Line Tool
Cuery is a command line tool designed to streamline the execution of commands using aliases defined in a configuration file. This tool provides a convenient way to manage and execute frequently used commands or complex sequences of actions, enhancing efficiency and productivity in command line environments.

## Installation
To use Cuery, simply clone this repository and compile the source code using Rust. Ensure that you have Rust and Cargo installed on your system.

``` bash
git clone <repository_url>
cd cuery
cargo build --release
```
OR
1. **Download the ZIP file**: 
   - Download the ZIP file containing the executable from the releases section of this repository.

2. **Extract the contents**:
   - Extract the contents of the ZIP file to a folder on your local machine.

3. **Move the executable to a folder in the system's PATH**:
   - Identify an appropriate folder in your system's PATH environment variable (e.g., `C:\Windows32` on Windows or `/usr/local/bin` on Unix-like systems).
   - Move the executable file and config folder (e.g., `cry.exe` on Windows or `cry` on Unix-like systems) to this folder.
   - Ensure that the folder containing the executable is added to the PATH variable, allowing you to run the CLI tool from any directory in the command line.

**Note**: If you're not familiar with setting up environment variables or configuring the system's PATH, you can simply move the executable to a folder you can easily access and run it from there. However, for optimal usage and convenience, it's recommended to set up the appropriate environment variable as described above.

## Usage
Cuery supports various commands and functionalities to facilitate command execution and configuration management. Below are the available commands:

### `cry run`
Run a command associated with a specific file extension.

``` bash
cry run <file_path>
```

### `cry list`
List mapped extensions or commands stored in the configuration file.

``` bash
cry list [-c|--list-commands] [-e|--list-extensions]
```
### `cry config`
Manage configuration settings.

#### `edit`
Edit the configuration file using the default text editor.

``` bash
cry config edit
```
#### `set`
Set a new alias for a command or file extension.

``` bash
cry config set <key> <value> [-c|--add-command] [-e|--add-extension]
```
#### `delete`
Delete an alias for a command or file extension.

``` bash
cry config delete <key> [-c|--delete-command] [-e|--delete-extension]
```
### Direct Command Execution
Execute a command directly using its alias.

``` bash
cry <alias>
```

## Configuration
Cuery uses a JSON configuration file to store aliases and their corresponding commands or file extensions. Below is an example of the configuration structure:

``` json
{
  "extensions": {
    "txt": "nano",
    "rs": "cargo run",
    "py": "python"
  },
  "commands": {
    "ls": "ls -al",
    "pwd": "pwd",
    "git": "git"
  }
}
```
## Contributing
Contributions to Cuery are welcome! Feel free to submit bug reports, feature requests, or pull requests via the [GitHub repository](https://github.com/Adriantobi/cuery).

## License
This project is licensed under the MIT License.
