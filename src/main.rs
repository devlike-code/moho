pub mod defaults;
pub mod grammar;
pub mod output;
pub mod parser;

use std::{
    ffi::{OsStr, OsString},
    fs,
    io::{self, Write},
    path::PathBuf,
    thread,
};

use clap::Parser;
use defaults::copy_default_files;
use dirs::config_dir;
use output::{write_file, OutputTemplate, OutputWriter, StringWriter};
use parser::{
    Argument, Block, Class, Declaration, Field, Method, MohoParser, Property, TranslationUnit,
    Type, Value,
};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(version)]
/// Moho: a gamedev-oriented code generator
pub struct CmdArguments {
    #[clap(index = 1)]
    /// directory to run generation in
    pub run_path: String,

    #[clap(long, short, default_value_t = config_dir()
        .map(|c| format!("{}{}.moho", c.as_os_str().to_str().unwrap().to_string(), std::path::MAIN_SEPARATOR))
        .unwrap_or(String::from(".")))]
    /// directory to find Moho config
    pub moho_path: String,

    #[clap(short, long, action, default_value_t = false)]
    /// open explorer to the moho config folder
    pub explore_configs: bool,

    #[clap(short, long, action, default_value_t = false)]
    pub copy_configs: bool,

    #[clap(short, long)]
    /// generate a class of this superclass from the command line
    pub generate_class: Option<String>,

    #[clap(short)]
    /// used in conjunction with --generate-class, names the new class without prompting
    pub new_class_name: Option<String>,
}

impl Default for CmdArguments {
    fn default() -> Self {
        Self {
            run_path: ".".into(),
            copy_configs: false,
            moho_path: config_dir()
                .map(|c| {
                    format!(
                        "{}{}.moho",
                        c.as_os_str().to_str().unwrap().to_string(),
                        std::path::MAIN_SEPARATOR
                    )
                })
                .unwrap_or(String::from(".")),
            explore_configs: false,
            generate_class: None,
            new_class_name: None,
        }
    }
}

fn main() -> std::io::Result<()> {
    let args = CmdArguments::parse();

    // Create moho directory if missing
    let moho_path = args.moho_path.clone();
    if std::fs::metadata(moho_path.as_str()).is_err() {
        let _ = std::fs::create_dir(moho_path.as_str());
        copy_default_files(&moho_path);
        println!("Copied default moho files to config path {}", moho_path);
    }

    if args.copy_configs {
        let _ = std::fs::create_dir(moho_path.as_str());
        copy_default_files(&moho_path);
        println!("Copied default moho files to config path {}", moho_path);
    }

    // If exploring, open explorer and quit.
    if args.explore_configs {
        open::that(args.moho_path)?;
        return Ok(());
    }

    if let Some(class) = args.generate_class {
        println!("Creating class as a subclass of {}", class);
        if std::fs::metadata(format!(
            "{}{}{}.rhai",
            &moho_path,
            std::path::MAIN_SEPARATOR,
            class
        ))
        .is_err()
        {
            println!("\n\tError: template {} not found. Quitting.", class);
            return Ok(());
        }

        let name = if args.new_class_name.is_none() {
            print!("  Class name: ");
            let _ = io::stdout().flush();
            {
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                name.trim().to_string()
            }
        } else {
            args.new_class_name.unwrap()
        };

        print!("Writing {}.moho... ", name);
        let _ = io::stdout().flush();
        let _ = fs::write(
            format!(
                "{}{}{}.moho",
                args.run_path,
                std::path::MAIN_SEPARATOR,
                name
            ),
            format!("class {} : {}\n{{\n\n}}\n", name, class),
        );
        println!("Done!");
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

fn append_to_path(p: impl Into<OsString>, s: impl AsRef<OsStr>) -> String {
    let mut p = p.into();
    p.push(std::path::MAIN_SEPARATOR_STR);
    p.push(s);
    let pathbuf: PathBuf = p.into();
    pathbuf
        .into_os_string()
        .into_string()
        .expect("Should be able to open path")
}

fn create_file(p: String) {
    let _ = fs::File::create(p).expect("Cannot create file, panicking!");
}

fn join_string_array(v: Vec<String>) -> String {
    v.join(", ")
}

fn join_property_array(v: Vec<Property>) -> String {
    join_string_array(
        v.iter()
            .map(|p| {
                if p.value.is_some() {
                    format!("{}={}", p.name, p.value.as_ref().unwrap())
                } else {
                    p.name.to_string()
                }
            })
            .collect::<Vec<_>>(),
    )
}

fn any_string_in_array(v: Vec<String>) -> bool {
    !v.is_empty()
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
    engine.register_fn("write_file", write_file);
    engine.register_fn("join", join_string_array);
    engine.register_fn("join", join_property_array);
    engine.register_fn("any", any_string_in_array);
    engine.build_type::<OutputWriter>();
    engine.build_type::<StringWriter>();
    engine.build_type::<OutputTemplate>();
    engine.build_type::<Class>();
    engine.build_type::<Block>();
    engine.build_type::<Property>();
    engine.build_type::<Field>();
    engine.build_type::<Type>();
    engine.build_type::<Value>();
    engine.build_type::<Method>();
    engine.build_type::<Argument>();

    engine.register_iterator::<Vec<String>>();
    engine.register_iterator::<Vec<Block>>();
    engine.register_iterator::<Vec<Field>>();
    engine.register_iterator::<Vec<Method>>();
    engine.register_iterator::<Vec<Argument>>();
    engine.register_iterator::<Vec<Property>>();
    engine.register_iterator::<Vec<Declaration>>();

    engine.register_type_with_name::<Declaration>("Declaration");
    engine.register_type_with_name::<TranslationUnit>("TranslationUnit");
    let mut scope = rhai::Scope::new();

    scope.push(
        "Output",
        OutputWriter::new(source_dir.clone(), moho_path.clone()),
    );
    scope.push_constant(
        "Filename",
        path.clone().file_name().map(|f| f.to_owned()).unwrap(),
    );
    scope.push_constant(
        "Path",
        source_dir
            .canonicalize()
            .unwrap()
            .as_os_str()
            .to_os_string()
            .into_string()
            .unwrap(),
    );

    for class in translation_unit.0 {
        let Class {
            name,
            inherit,
            inner,
        } = class.clone();

        scope.push_constant("Fields", inner.clone().fields());
        scope.push_constant("Methods", inner.clone().methods());
        scope.push_constant("Name", name.clone());
        scope.push_constant("ClassProperties", inner.properties.clone());
        scope.push_constant("Inherit", inherit.first().cloned());

        let mut tail = inherit.clone();
        tail.reverse();
        tail.pop();
        tail.reverse();
        scope.push_constant("OtherInherits", tail);

        let script_file = append_to_path(
            moho_path.clone(),
            inherit
                .first()
                .cloned()
                .map(|f| f + ".rhai")
                .unwrap_or("base.rhai".to_string()),
        );

        if let Err(result) = engine.run_file_with_scope(&mut scope, PathBuf::from(script_file)) {
            println!(
                "Failed to execute moho on file {}: {:?}",
                name.clone(),
                result
            );

            return;
        }
    }
}
