use anyhow::Result;
use colored::*;
use dialoguer::Select;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() -> Result<()> {
    // 1. Get folder from CLI args
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("{}", "Usage: autoext <folder>".red());
        std::process::exit(1);
    }
    let watch_path = PathBuf::from(&args[1]);
    println!("{} {:?}", "📂 Watching folder:".green().bold(), watch_path);

    // 2. Create a channel for events
    let (tx, rx) = channel();

    // 3. Create watcher
    let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
    watcher.watch(&watch_path, RecursiveMode::NonRecursive)?;

    // 4. Wait for first file creation
    loop {
        match rx.recv_timeout(Duration::from_secs(60)) {
            Ok(Ok(event)) => {
                if let EventKind::Create(_) = event.kind {
                    if let Some(path) = event.paths.get(0) {
                        println!(
                            "{} {:?}",
                            "🆕 New file detected:".yellow().bold(),
                            path.file_name().unwrap()
                        );

                        // 5. Prompt user for extension
                        let formats = vec!["Rust", "Python", "HTML", "JavaScript", "C++" 
                            , "Other"];
                        let selection = Select::new()
                            .with_prompt("Choose file format".cyan().bold().to_string())
                            .items(&formats)
                            .default(0)
                            .interact()?;

                        let ext = match formats[selection] {
                            "Rust" => "rs",
                            "Python" => "py",
                            "HTML" => "html",
                            "JavaScript" => "js",
                            "C++" => "cpp",
                            "Other" => {
                                let custom_ext: String = dialoguer::Input::new()
                                    .with_prompt("Enter custom file extension (without dot)")
                                    .interact_text()?;
                                if custom_ext.is_empty() {
                                    eprintln!("{}", "❌ Empty extension provided, using default 'txt'.".red());
                                    "txt"
                                } else {
                                    &custom_ext.clone()
                                }
                            }
                            _ => "txt",
                        };

                        // 6. Rename file
                        let mut new_path = path.clone();
                        new_path.set_extension(ext);
                        fs::rename(&path, &new_path)?;

                        println!(
                            "{} {:?}",
                            "✅ Renamed to:".green().bold(),
                            new_path.file_name().unwrap()
                        );
                        break; // exit after first file for MVP
                    }
                }
            }
            Ok(Err(e)) => eprintln!("{} {:?}", "❌ Watch error:".red(), e),
            Err(_) => {
                println!("{}", "⚠️  No new file detected in time, exiting.".yellow());
                break;
            }
        }
    }

    Ok(())
}
