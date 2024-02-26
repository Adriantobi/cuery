use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use which::which;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[command(name = "cry", author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Commands>,
    direct_command: Option<String>
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    #[clap(about="Run command")]
    Run {
        #[clap(help="File Path")]
        file_path: String
    },
    #[clap(about="List commands or extensions")]
    List {
        #[clap(long, short = 'c', default_value = "false", help = "List commands")]
        #[clap(alias = "commands")]
        list_commands: bool,
        #[clap(long, short = 'e', default_value = "false", help = "List extensions")]
        #[clap(alias = "extensions")]
        list_extensions: bool,
    },
    #[command(subcommand)]
    #[clap(about = "Configuration subcommands")]
    Config(ConfigSubcommand),
}

#[derive(Parser, Debug, Clone)]
enum ConfigSubcommand {
    #[clap(about="Edit configuration")]
    Edit,
    #[clap(about="Set configuration")]
    Set {
        key: String,
        value: String,
        #[clap(long, short = 'e', default_value = "false", help = "Add extension")]
        #[clap(alias = "extensions")]
        add_extension: bool,
        #[clap(long, short = 'c', default_value = "false", help = "Add command")]
        #[clap(alias = "commands")]
        add_command: bool
    },
    #[clap(about="Delete command or extension")]
    Delete {
        key: String,
        #[clap(long, short = 'e', default_value = "false", help = "Delete extension")]
        #[clap(alias = "extensions")]
        delete_extension: bool,
        #[clap(long, short = 'c', default_value = "false", help = "Delete command")]
        #[clap(alias = "commands")]
        delete_command: bool
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    extensions: HashMap<String, String>,
    commands: HashMap<String, String>
}

fn read_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(file_path)?;

    // Deserialize the JSON into the Config struct
    let config: Config = serde_json::from_str(&contents)?;

    // println!("Read config: {:?}", config);

    Ok(config)
}

fn run_code(value: String, config: &Config) {
    // Extract file extension
    let file_extension = match value.split('.').last() {
        Some(ext) => ext,
        None => {
            println!("Invalid file type");
            return;
        }
    };

    // Check if the extension exists in the commands
    if let Some(command) = config.extensions.get(file_extension) {
        // Execute the command
        println!("Running command for file extension '{}': {}", file_extension, command);
        let split_command: Vec<&str> = command.split_whitespace().collect();
        let mut cmd = std::process::Command::new(which(split_command.first().map(|s| s.to_string()).unwrap_or_else(String::new)).unwrap());
    
        for arg in split_command.iter().skip(1) {
            cmd.arg(arg);
        }

        cmd.arg(value);

        cmd.spawn().expect("Failed to execute command");
    } else {
        println!("No command found for file extension: {}", file_extension);
    }
}

fn list(config: &Config, list_extensions: bool, list_commands: bool) {
    if list_extensions && !list_commands {
        return_extensions(config);
    } else if list_commands && !list_extensions {
        return_commands(config);
    } else {
        return_extensions(config);
        println!("\n");
        return_commands(config);
    }
}

fn return_extensions(config: &Config) {
    println!("Mapped Extensions:");
    for (extension, command) in &config.extensions {
        println!("  {}: {}", extension, command);
    }
}

fn return_commands(config: &Config) {
    println!("Mapped Commands:");
    for (short_hand, command) in &config.commands {
        println!("  {}: {}", short_hand, command);
    }
}

fn open_config(file_path: &str) -> Result<(), std::io::Error> {
    let editor = if cfg!(windows) {
        "notepad"
    } else if cfg!(target_os = "macos") {
        "open"
    } else {
        "gedit"
    };

    std::process::Command::new(editor)
        .arg(file_path)
        .spawn()?
        .wait()?;

    Ok(())
}

fn write_config(config: &Config, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let serialized_config = serde_json::to_string_pretty(config)?;
    std::fs::write(file_path, serialized_config)?;
    Ok(())
}

fn execute_command(command: &str) {
    println!("Executing command: {}", command);
    // Execute the command using appropriate mechanism (e.g., shell execution)
    let split_command: Vec<&str> = command.split_whitespace().collect();
    let mut cmd = std::process::Command::new(which(split_command.first().map(|s| s.to_string()).unwrap_or_else(String::new)).unwrap());
    
    for arg in split_command.iter().skip(1) {
        cmd.arg(arg);
    }

    let _ = cmd
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()
        .expect("Failed to execute command")
        .wait_with_output()
        .expect("Failed to wait for command completion");
}

fn main() {
    let mut file_path = std::env::current_exe().expect("Failed to get current executable path");
    // Remove the binary filename from the path
    file_path.pop();
    // Append the configuration file name
    file_path.push("config");
    file_path.push("config.json");

    if !file_path.exists() {
        eprintln!("Error: Configuration file not found at {:?}", file_path);
        return;
    }

    let file_path_str = file_path.as_path().display().to_string();

    let cli = Cli::parse();
    let mut config = match read_config(&file_path_str) {
        Ok(cfg) => cfg,
        Err(err) => {
            eprintln!("Error: Couldn't read config: {}", err);
            return;
        }
    };

    if let Some(cmd) = cli.cmd {
        match cmd {
            Commands::Run { file_path } => run_code(file_path, &config),
            Commands::List { list_extensions, list_commands } => list(&config, list_extensions, list_commands),
            Commands::Config(subcommand) => match subcommand {
                ConfigSubcommand::Edit => {
                    if let Err(err) = open_config(&file_path_str) {
                        eprintln!("Error opening config with default editor: {}", err);
                    }
                },
                ConfigSubcommand::Set { key, value, add_command, add_extension } => {
                    if let Some(existing_value) = config.extensions.get(&key) {
                        println!("Warning: Extension '{}' already exists with value '{}'. Overwriting...", key, existing_value);
                    } else if let Some(used_value) = config.commands.get(&key) {
                        println!("Warning: Command '{}' already exists with value '{}'. Overwriting...", key, used_value);
                    }

                    let key_clone = key.clone();
                    let value_clone = value.clone();
                    if add_command && !add_extension {
                        config.commands.insert(key_clone, value_clone);
                    } else if add_extension && !add_command {
                        config.extensions.insert(key_clone, value_clone);
                    } else if add_extension && add_command {
                        eprintln!("Error: Cannot add both commands and extensions simultaneously. Please choose one.");
                    } else {
                        eprintln!("Error: Please specify whether to add a command or an extension.");
                    }
                    // Write the modified config back to the file
                    if let Err(err) = write_config(&config, &file_path_str) {
                        eprintln!("Error writing config file: {}", err);
                    } else {
                        println!("Mapped in config: {} -> {}", key, value);
                    }
                },
                ConfigSubcommand::Delete { key, delete_extension, delete_command } => {
                    if delete_extension && !delete_command {
                        if let Some(existing_value) = config.extensions.remove(&key) {
                            println!("Deleted Extension '{}' with value '{}'.", key, existing_value);
                        } else {
                            println!("Extension '{}' not found.", key);
                        }
                    }

                    if delete_command && !delete_extension {
                        if let Some(existing_value) = config.commands.remove(&key) {
                            println!("Deleted Command '{}' with value '{}'.", key, existing_value);
                        } else {
                            println!("Command '{}' not found.", key);
                        }
                    }

                    if delete_extension && delete_command {
                        eprintln!("Error: Cannot remove both commands and extensions simultaneously. Please choose one.");
                    }

                    else {
                        eprintln!("Error: Please specify whether to delete a command or an extension.");
                    }

                    // Write the modified config back to the file
                    if let Err(err) = write_config(&config, &file_path_str) {
                        eprintln!("Error writing config file: {}", err);
                    }
                }
            },
        }
    } else if let Some(command) = cli.direct_command {
        // Check if the entered command matches any key in the config file
        if let Some(command_str) = config.commands.get(&command) {
            execute_command(command_str);
        } else {
            eprintln!("Command {:?} not found in config.", command);
        }
    } 
}
