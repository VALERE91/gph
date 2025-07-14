// This file defines the command-line arguments and subcommands using clap.
use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// A CLI to manage university game projects
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Configure or view the global settings for gph.
    /// Running `gph config` without a subcommand will display the current configuration.
    Config {
        #[command(subcommand)]
        command: Option<ConfigCommands>,
    },
    /// Initialize a project in the given directory
    Init {
        /// The path to the game project directory. Defaults to the current directory.
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Build the project at the given path
    Build {
        /// The path to the initialized project directory. Defaults to the current directory.
        #[arg(default_value = ".")]
        path: PathBuf,
    },
    /// Package the project at the given path
    Package {
        /// The path to the initialized project directory. Defaults to the current directory.
        #[arg(default_value = ".")]
        path: PathBuf,
        /// The output path for the packaged build.
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Manage or view engine configurations.
    /// Running `gph config engine` will display the engine path configuration.
    Engine {
        #[command(subcommand)]
        command: Option<EngineCommands>,
    },
}

#[derive(Subcommand)]
pub enum EngineCommands {
    /// Add or update the path to an engine executable/buildtool
    Add {
        /// The type of engine to configure
        #[arg(value_enum)]
        engine_type: EngineType,
        /// The path to the engine's executable or build tool
        path: PathBuf,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum EngineType {
    Unreal,
    Unity,
    Godot,
}