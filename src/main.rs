use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use colored::*;
use console::Term;
use inquire::{Password, Select, Text};
use std::path::{Path, PathBuf};
use std::process::exit;

mod archive;
mod crypto;

// --- NERD FONT ICONS ---
mod icons {
    pub const LOCK: &str = "\u{f023}";
    pub const UNLOCK: &str = "\u{f13e}";
    pub const ARCHIVE: &str = "\u{f1c6}";
    pub const FOLDER: &str = "\u{f07b}";
    pub const KEY: &str = "\u{f084}";
    pub const CHECK: &str = "\u{f00c}";
    pub const ERROR: &str = "\u{f00d}";
    pub const TERMINAL: &str = "\u{f120}";
}

// --- CATPPUCCIN MACCHIATO THEME ---
trait MacchiatoTheme {
    fn c_mauve(self) -> ColoredString;
    fn c_pink(self) -> ColoredString;
    fn c_blue(self) -> ColoredString;
    fn c_green(self) -> ColoredString;
    fn c_yellow(self) -> ColoredString;
    // c_red removed to fix unused warning
    fn c_text(self) -> ColoredString;
    fn c_subtext(self) -> ColoredString;
}

impl MacchiatoTheme for &str {
    fn c_mauve(self) -> ColoredString { self.truecolor(198, 160, 246) }
    fn c_pink(self) -> ColoredString { self.truecolor(245, 189, 230) }
    fn c_blue(self) -> ColoredString { self.truecolor(138, 173, 244) }
    fn c_green(self) -> ColoredString { self.truecolor(166, 218, 149) }
    fn c_yellow(self) -> ColoredString { self.truecolor(238, 212, 159) }
    // c_red implementation removed
    fn c_text(self) -> ColoredString { self.truecolor(202, 211, 245) }
    fn c_subtext(self) -> ColoredString { self.truecolor(165, 173, 203) }
}

#[derive(Parser)]
#[command(name = "shadowenv")]
#[command(about = "Secure Markdown Archival System", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Encrypt {
        #[arg(value_name = "FOLDER_PATH")]
        input_path: PathBuf,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    Decrypt {
        #[arg(value_name = "SHADOW_FILE")]
        input_path: PathBuf,
        #[arg(short, long)]
        output_dir: Option<PathBuf>,
    },
}

// --- HELPER: EXPAND & RESOLVE PATH ---
fn resolve_path(path_str: &str) -> PathBuf {
    let path_str = path_str.trim();
    let mut path = PathBuf::from(path_str);

    // 1. Handle Tilde Expansion
    if path_str.starts_with("~/") || path_str == "~" {
        if let Ok(home) = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
            let suffix = if path_str == "~" { "" } else { &path_str[2..] };
            path = PathBuf::from(home).join(suffix);
        }
    }

    // 2. Canonicalize (Resolve relative paths like ./ or ../ to absolute)
    if let Ok(canon) = path.canonicalize() {
        return canon;
    }

    path
}

fn main() -> Result<()> {
    if std::env::args().len() == 1 {
        return run_interactive_mode();
    }

    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Encrypt { input_path, output }) => {
            run_encrypt(&input_path, output, None)?
        }
        Some(Commands::Decrypt { input_path, output_dir }) => {
            run_decrypt(&input_path, output_dir, None)?
        }
        None => unreachable!(),
    }

    Ok(())
}

fn run_interactive_mode() -> Result<()> {
    let term = Term::stdout();
    term.clear_screen()?;

    print_header();

    let opt_encrypt = format!("{} Encrypt Folder", icons::LOCK);
    let opt_decrypt = format!("{} Decrypt Archive", icons::UNLOCK);
    let opt_exit = format!("{} Exit", icons::ERROR);

    let options = vec![opt_encrypt.as_str(), opt_decrypt.as_str(), opt_exit.as_str()];
    
    println!("{}", "Select Protocol ::".c_subtext());
    let choice = Select::new("", options).with_help_message("Use arrow keys to navigate").prompt()?;

    if choice == opt_encrypt {
        println!("\n{}", "Target Configuration ::".c_subtext());
        let input = Text::new("Path:")
            .with_placeholder("e.g. ~/projects/my_secrets")
            .with_validator(|input: &str| {
                let path = resolve_path(input);
                if path.is_dir() {
                    Ok(inquire::validator::Validation::Valid)
                } else {
                    Ok(inquire::validator::Validation::Invalid(
                        format!("Path not found: {:?}", path).into(),
                    ))
                }
            })
            .prompt()?;
        
        let input_path = resolve_path(&input);

        let password = Password::new("Passphrase:")
            .with_display_mode(inquire::PasswordDisplayMode::Masked)
            .with_custom_confirmation_message("Verify:")
            .with_custom_confirmation_error_message("Passphrases do not match")
            .prompt()?;

        run_encrypt(&input_path, None, Some(password))?;

    } else if choice == opt_decrypt {
        println!("\n{}", "Target Configuration ::".c_subtext());
        let input = Text::new("Shadow File:")
            .with_validator(|input: &str| {
                let path = resolve_path(input);
                if path.is_file() {
                    if path.extension().map_or(false, |ext| ext == "shadow") {
                        Ok(inquire::validator::Validation::Valid)
                    } else {
                        Ok(inquire::validator::Validation::Invalid(
                             "File exists but extension is not .shadow".into()
                        ))
                    }
                } else {
                    Ok(inquire::validator::Validation::Invalid(
                        format!("File not found: {:?}", path).into(),
                    ))
                }
            })
            .prompt()?;

        let input_path = resolve_path(&input);
        
        let default_out_name = format!("{}_restored", input_path.file_stem().unwrap().to_string_lossy());
        let parent_dir = input_path.parent().unwrap_or(Path::new("."));
        let default_out_path = parent_dir.join(&default_out_name);

        let out_str = Text::new("Destination:")
            .with_default(&default_out_path.to_string_lossy())
            .prompt()?;
        
        let output_dir = resolve_path(&out_str);

        let password = Password::new("Passphrase:")
            .with_display_mode(inquire::PasswordDisplayMode::Masked)
            .without_confirmation()
            .prompt()?;

        run_decrypt(&input_path, Some(output_dir), Some(password))?;

    } else {
        exit(0);
    }

    Ok(())
}

fn print_header() {
    println!("{}", r#"
                                                           
▄█████ ▄▄ ▄▄  ▄▄▄  ▄▄▄▄   ▄▄▄  ▄▄   ▄▄ ██████ ▄▄  ▄▄ ▄▄ ▄▄ 
▀▀▀▄▄▄ ██▄██ ██▀██ ██▀██ ██▀██ ██ ▄ ██ ██▄▄   ███▄██ ██▄██ 
█████▀ ██ ██ ██▀██ ████▀ ▀███▀  ▀█▀█▀  ██▄▄▄▄ ██ ▀██  ▀█▀  
                                                           
██████ ▄▄  ▄▄  ▄▄▄▄ ▄▄▄▄  ▄▄ ▄▄ ▄▄▄▄ ▄▄▄▄▄▄ ▄▄  ▄▄▄  ▄▄  ▄▄ 
██▄▄   ███▄██ ██▀▀▀ ██▄█▄ ▀███▀ ██▄█▀  ██   ██ ██▀██ ███▄██ 
██▄▄▄▄ ██ ▀██ ▀████ ██ ██   █   ██     ██   ██ ▀███▀ ██ ▀██ 
                                                            

  "#.c_mauve().bold());
    
    println!("   {} {}\n", icons::TERMINAL.c_pink(), "v1.0 :: SHADOW ENV".c_subtext());
}

fn run_encrypt(input: &Path, output: Option<PathBuf>, password_opt: Option<String>) -> Result<()> {
    let folder_name = input.file_name()
        .ok_or_else(|| anyhow::anyhow!("Invalid input path"))?
        .to_string_lossy();

    let output_path = output.unwrap_or_else(|| {
        input.join(format!("{}.shadow", folder_name))
    });

    let password = match password_opt {
        Some(p) => p,
        None => {
            eprint!("{} Enter passphrase: ", icons::KEY);
            let p = rpassword::read_password()?;
            eprint!("{} Confirm: ", icons::KEY);
            if rpassword::read_password()? != p {
                anyhow::bail!("Passphrase mismatch");
            }
            p
        }
    };

    println!("\n{} {} {}", icons::FOLDER.c_blue(), "Packing system:".c_text(), input.display().to_string().c_pink());
    
    let archive_data = archive::create_archive(input, &output_path)
        .context("Failed to create archive")?;

    println!("{} {}", icons::LOCK.c_yellow(), "Encrypting stream...".c_text());

    crypto::encrypt_data(&archive_data, &password, &output_path)
        .context("Encryption failed")?;

    println!("\n{} {} {:?}", icons::CHECK.c_green(), "SECURE OBJECT CREATED ::".c_green().bold(), output_path);
    Ok(())
}

fn run_decrypt(input: &Path, output_dir: Option<PathBuf>, password_opt: Option<String>) -> Result<()> {
    let out_dir = output_dir.unwrap_or_else(|| PathBuf::from("."));
    
    if !out_dir.exists() {
        std::fs::create_dir_all(&out_dir)
            .context(format!("Failed to create output directory: {:?}", out_dir))?;
    }

    let password = match password_opt {
        Some(p) => p,
        None => {
            eprint!("{} Enter passphrase: ", icons::KEY);
            rpassword::read_password()?
        }
    };

    println!("\n{} {}", icons::KEY.c_pink(), "Authenticating...".c_text());

    let decrypted_data = crypto::decrypt_data(input, &password)
        .context("Access Denied: Invalid credentials or data corruption.")?;

    println!("{} {}", icons::ARCHIVE.c_yellow(), "Unpacking filesystem...".c_text());

    archive::unpack_archive(&decrypted_data, &out_dir)?;

    println!("\n{} {} {:?}", icons::CHECK.c_green(), "SYSTEM RESTORED ::".c_green().bold(), out_dir);
    Ok(())
}
