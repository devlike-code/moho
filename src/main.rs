pub mod ast;
pub mod parser;

use std::{
    ffi::{OsStr, OsString},
    fs::{self},
    path::PathBuf,
    process::Command,
    thread::{self},
};

use clap::Parser;
use dirs::config_dir;
use parser::{Class, MohoParser};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(version)]
/// Moho: a gamedev-oriented code generator
pub struct Arguments {
    #[clap(long, short, default_value_t = String::from("."))]
    /// directory to run generation in
    pub run_path: String,

    #[clap(long, short, default_value_t = config_dir()
        .map(|c| c.as_os_str().to_str().unwrap().to_string() + "\\.moho")
        .unwrap_or(String::from(".")))]
    /// directory to find Moho config
    pub moho_path: String,
    #[clap(short, long, action, default_value_t = false)]
    pub explore_configs: bool,
}

impl Default for Arguments {
    fn default() -> Self {
        Self {
            run_path: ".".into(),
            moho_path: format!(
                "{}\\.moho",
                config_dir()
                    .map(|c| c.as_os_str().to_str().unwrap().to_string() + "\\.moho")
                    .unwrap_or(String::from("."))
            ),
            explore_configs: false,
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = Arguments::parse();

    // Create moho directory if missing
    let moho_path = args.moho_path.clone();
    if std::fs::metadata(moho_path.as_str()).is_err() {
        std::fs::create_dir(moho_path.as_str())?;
        create_file(append_to_path(moho_path, "empty.rhai"));
    }

    // If exploring, open explorer and quit.
    if args.explore_configs {
        Command::new("explorer")
            .arg(args.moho_path)
            .spawn()
            .unwrap();
        return Ok(());
    }

    // Do default stuff
    let mut handles = vec![];
    for entry in WalkDir::new(args.run_path) {
        let Ok(entry) = entry else {
            continue;
        };

        if entry
            .path()
            .extension()
            .map(|e| e == "moho")
            .unwrap_or_default()
        {
            let path = entry.path().to_path_buf();
            let moho_path = args.moho_path.clone();
            handles.push(thread::spawn(move || run_moho(path, moho_path.into())));
        }
    }

    for handle in handles {
        let _ = handle.join();
    }

    Ok(())
}

fn append_to_path(p: impl Into<OsString>, s: impl AsRef<OsStr>) -> PathBuf {
    let mut p = p.into();
    p.push("\\");
    p.push(s);
    p.into()
}

fn create_file(p: PathBuf) {
    let _ = fs::File::create(p).expect("Cannot create file, panicking!");
}

fn run_moho(path: PathBuf, moho_path: PathBuf) {
    let Ok(input) = fs::read_to_string(path.clone()) else {
        return;
    };

    let Ok(translation_unit) = MohoParser::apply(&input) else {
        return;
    };

    let Some(source_dir) = path.clone().parent().map(|p| p.to_path_buf()) else {
        return;
    };

    let mut engine = rhai::Engine::new();
    engine.register_fn("create_file", create_file);

    let mut scope = rhai::Scope::new();

    scope.push_constant(
        "Filename",
        path.clone().file_name().map(|f| f.to_owned()).unwrap(),
    );
    scope.push_constant("Path", source_dir);

    for Class {
        name,
        inherit,
        inner,
    } in translation_unit.0
    {
        scope.push_constant("Name", name);
        scope.push_constant("Inherit", inherit.clone());
        scope.push_constant("Class", inner);

        let script_file = append_to_path(
            moho_path.clone(),
            inherit
                .first()
                .cloned()
                .map(|f| f + ".rhai")
                .unwrap_or("empty.rhai".to_string()),
        );

        if let Err(result) = engine.run_file_with_scope(&mut scope, script_file) {
            println!("Failed to moho: {:?}", result);
        }
    }
}
