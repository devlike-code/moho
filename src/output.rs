use std::{collections::HashMap, fs, path::PathBuf};

use rhai::CustomType;
use string_template::Template;

pub fn write_file(p: String, content: String) {
    fs::write(p.clone(), content).unwrap_or_else(|_| panic!("Cannot write file {}", p));
}

#[derive(Default, Clone)]
pub struct StringWriter {
    text: String,
}

impl StringWriter {
    pub fn add(&mut self, txt: String) {
        self.text += &txt;
    }

    pub fn add_string_writer(&mut self, txt: StringWriter) {
        self.text += &txt.get();
    }

    pub fn get(&self) -> String {
        self.text.clone()
    }
}

impl CustomType for StringWriter {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name("StringWriter")
            .with_fn("add", StringWriter::add)
            .with_fn("add", StringWriter::add_string_writer)
            .with_fn("get", StringWriter::get);
    }
}

#[derive(Clone)]
pub struct OutputWriter {
    output_path: String,
    config_path: String,
    text: String,
}

macro_rules! path_to_str {
    ($name:ident) => {
        $name
            .canonicalize()
            .unwrap()
            .as_os_str()
            .to_os_string()
            .into_string()
            .unwrap()
    };
}

impl OutputWriter {
    pub fn new(output_path: PathBuf, config_path: PathBuf) -> Self {
        OutputWriter {
            output_path: path_to_str!(output_path),
            config_path: path_to_str!(config_path),
            text: "".into(),
        }
    }

    pub fn add(&mut self, txt: String) {
        self.text += &txt;
    }

    pub fn get(&self) -> String {
        self.text.clone()
    }

    pub fn has_template(&mut self, temp: String) -> bool {
        let file = format!("{}\\{}", self.config_path, temp);
        fs::metadata(file).is_ok()
    }

    pub fn template_from_file(&mut self, temp: String) -> OutputTemplate {
        let file = format!("{}\\{}", self.config_path, temp);
        OutputTemplate {
            pattern: fs::read_to_string(file.clone())
                .unwrap_or_else(|_| panic!("File not found {:?}", file)),
            vars: HashMap::default(),
        }
    }

    pub fn embed(&mut self, temp: OutputTemplate) {
        self.add(temp.finish());
    }

    pub fn write_to(&mut self, path: String) {
        write_file(format!("{}\\{}", self.output_path, path), self.get());
    }

    pub fn clear(&mut self) {
        self.text = "".into();
    }

    pub fn snippet(&mut self) -> StringWriter {
        StringWriter::default()
    }
}

impl CustomType for OutputWriter {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name("OutputWriter")
            .with_fn("add", OutputWriter::add)
            .with_fn("has_part", OutputWriter::has_template)
            .with_fn("part", OutputWriter::template_from_file)
            .with_fn("embed", OutputWriter::embed)
            .with_fn("write_to", OutputWriter::write_to)
            .with_fn("clear", OutputWriter::clear)
            .with_fn("snippet", OutputWriter::snippet);
    }
}

#[derive(Default, Clone)]
pub struct OutputTemplate {
    pattern: String,
    vars: HashMap<String, String>,
}

impl OutputTemplate {
    pub fn put(&mut self, name: &str, value: &str) {
        self.vars.insert(name.to_owned(), value.to_owned());
    }

    pub fn put_string_writer(&mut self, name: &str, value: StringWriter) {
        self.vars.insert(name.to_owned(), value.get());
    }

    pub fn finish(self) -> String {
        let template = Template::new(self.pattern.as_str());
        template.render(
            &self
                .vars
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect(),
        )
    }
}

impl CustomType for OutputTemplate {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name("OutputTemplate")
            .with_fn("put", OutputTemplate::put)
            .with_fn("put", OutputTemplate::put_string_writer)
            .with_fn("finish", OutputTemplate::finish);
    }
}
