pub mod args;

use gph_core::ProjectManager;
use args::{Cli, Commands, ConfigCommands, EngineCommands, EngineType};
use clap::Parser;
use gph_core::config::GlobalConfig;

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
        Commands::Config { command } => match command {
            // `gph config` was run
            None => {
                println!("Displaying global configuration:");
                let config_str =
                    toml::to_string_pretty(&global_config).expect("Failed to serialize config");
                println!("{}", config_str);
            }
            Some(ConfigCommands::Engine { command }) => match command {
                // `gph config engine` was run
                None => {
                    println!("Displaying engine configuration:");
                    let engine_config_str = toml::to_string_pretty(&global_config.engine_paths)
                        .expect("Failed to serialize engine config");
                    println!("{}", engine_config_str);
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
                }
            },
        },
        Commands::Init { path } => {
            let manager = ProjectManager::new(global_config);
            manager.init_project(&path).map(|_| ())?;
        }
        Commands::Build { path } => {
            let _manager = ProjectManager::new(global_config);
            // In a real app, you'd load the project config and find the right engine
            // to call `build_project` on.
            println!("Build command would run for project at: {}", path.display());
        }
        Commands::Package { path, output } => {
            let _manager = ProjectManager::new(global_config);

            let output_path = output.unwrap_or_else(|| {
                let timestamp = chrono::Local::now().format("%Y%m%d-%H%M%S").to_string();
                path.join(".gph").join("packages").join(timestamp)
            });

            // In a real app, you'd load project config, find the right engine,
            // and call `package_project`.
            println!(
                "Package command would run for project at: {}",
                path.display()
            );
            println!("Output would be saved to: {}", output_path.display());
            std::fs::create_dir_all(&output_path)?;
        }
    };

    Ok(())
}