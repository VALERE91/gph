pub mod args;

use std::fs;
use std::path::Path;
use gph_core::ProjectManager;
use args::{Cli, Commands, ConfigCommands, EngineCommands, EngineType};
use clap::Parser;
use gph_core::config::{GlobalConfig, ProjectConfig};
use gph_core::error::Error;

/// The main entry point for the CLI logic.
pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // --- Global Config Handling ---
    let config_path = dirs::config_dir()
        .expect("Could not find a valid config directory")
        .join("gph/config.toml");

    let mut global_config = GlobalConfig::load(&config_path).unwrap_or_default();

    // --- Command Matching ---
    match cli.command {
        Commands::Config { command } => handle_config_command(command, &mut global_config, &config_path),
        Commands::Init { path } => {
            let manager = ProjectManager::new(global_config);
            manager.init_project(&path).map(|_| ())?;
            Ok(())
        }
        Commands::Build { path } => {
            let manager = ProjectManager::new(global_config);
            let project_config = ProjectConfig::load(&path)?;

            let engine_type = project_config.engine_type.as_ref().ok_or(Error::EngineTypeNotSpecified)?;
            let engine = manager.get_engine(engine_type).ok_or_else(|| Error::UnsupportedEngine(engine_type.clone()))?;

            let projects = engine.detect_projects(&path)?;
            // For now, just build the first project found in the directory
            if let Some(project_info) = projects.first() {
                engine.build_project(project_info, &project_config)?;
                Ok(())
            } else {
                println!("No {} project found at path '{}'", engine_type, path.display());
                Ok(())
            }
        }
        Commands::Package { path, output } => {
            let manager = ProjectManager::new(global_config);
            let project_config = ProjectConfig::load(&path)?;

            let output_path = output.unwrap_or_else(|| {
                let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
                path.join(".gph").join("packages").join(timestamp)
            });
            fs::create_dir_all(&output_path)?;

            let engine_type = project_config.engine_type.as_ref().ok_or(Error::EngineTypeNotSpecified)?;
            let engine = manager.get_engine(engine_type).ok_or_else(|| Error::UnsupportedEngine(engine_type.clone()))?;

            let projects = engine.detect_projects(&path)?;
            if let Some(project_info) = projects.first() {
                engine.package_project(project_info, &project_config, &output_path).map(|_|())?;
                Ok(())
            } else {
                println!("No {} project found at path '{}'", engine_type, path.display());
                Ok(())
            }
        }
    }
}

fn handle_config_command(command: Option<ConfigCommands>,
                 global_config: &mut GlobalConfig,
                 config_path: &Path) -> anyhow::Result<()>
{
    match command {
        // `gph config` was run
        None => {
            println!("Displaying global configuration:");
            let config_str =
                toml::to_string_pretty(&global_config).expect("Failed to serialize config");
            println!("{}", config_str);
            Ok(())
        }
        Some(ConfigCommands::Engine { command }) => match command {
            // `gph config engine` was run
            None => {
                println!("Displaying engine configuration:");
                let engine_config_str = toml::to_string_pretty(&global_config.engine_paths)
                    .expect("Failed to serialize engine config");
                println!("{}", engine_config_str);
                Ok(())
            }
            // `gph config engine add ...` was run
            Some(EngineCommands::Add { engine_type, path }) => {
                println!("Updating global config...");
                match engine_type {
                    EngineType::Unreal => global_config.engine_paths.unreal = Some(path),
                    EngineType::Unity => global_config.engine_paths.unity = Some(path),
                    EngineType::Godot => global_config.engine_paths.godot = Some(path),
                }
                global_config.save(&config_path)?;
                Ok(())
            }
        }
    }
}