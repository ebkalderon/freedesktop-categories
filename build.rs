extern crate curl;
extern crate minidom;
extern crate phf_codegen;

use std::env::{self, VarError};
use std::fs::{self, File};
use std::io::{BufWriter, Error as IoError, Write};
use std::path::Path;

use curl::easy::Easy;
use minidom::Element;
use phf_codegen::Map;

const SPEC_URL: &str = "https://specifications.freedesktop.org/menu-spec/menu-spec-latest.xml";
const OUT_FILE: &str = "menu-spec-latest.xml";
const CATEGORIES_TYPE_SIG: &str = "pub static CATEGORIES: ::phf::Map<&'static str, Category> = ";

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Var(VarError),
}

fn main() {
    // `quick-xml` does not support these `ENTITY` symbols, so strip them out before parsing.
    let xml = fetch_or_download()
        .expect("Unable to fetch and load Freedesktop XML")
        .replace("&version", "version")
        .replace("&dtd-version", "dtd-version");

    let spec: Element = xml
        .parse()
        .expect(&format!("Failed to parse `{}`", OUT_FILE));

    let categories = spec
        .children()
        .filter(|child| child.name() == "appendix")
        .find(|child| {
            child
                .attr("id")
                .map(|id| id == "category-registry")
                .is_some()
        })
        .expect("Could not locate `category-registry` appendix in Freedesktop XML spec");

    let map = gen_category_map(categories).unwrap();

    let out = Path::new(&env::var("OUT_DIR").unwrap()).join("map.rs");
    let mut file = BufWriter::new(File::create(&out).unwrap());
    write!(file, "{}", CATEGORIES_TYPE_SIG).unwrap();
    map.build(&mut file).unwrap();
    write!(file, ";\n").unwrap();
}

fn fetch_or_download() -> Result<String, Error> {
    let out_dir = env::var("OUT_DIR").map_err(Error::Var)?;
    let path = Path::new(&out_dir).join(OUT_FILE);

    if !path.exists() {
        let mut data = Vec::new();
        let mut handle = Easy::new();
        handle.url(SPEC_URL).unwrap();
        {
            let mut transfer = handle.transfer();
            transfer
                .write_function(|new_data| {
                    data.extend_from_slice(new_data);
                    Ok(new_data.len())
                })
                .unwrap();
            transfer.perform().unwrap();
        }

        fs::write(&path, &data).map_err(Error::Io)?;
    }

    fs::read_to_string(&path).map_err(Error::Io)
}

fn gen_category_map(categories: &Element) -> Result<Map<String>, Error> {
    let mut map = phf_codegen::Map::new();

    let main_section = categories
        .children()
        .filter(|child| child.name() == "sect1")
        .find(|child| {
            child
                .attr("id")
                .map(|id| id == "main-category-registry")
                .is_some()
        })
        .and_then(|main| {
            main.children().find(|child| {
                child
                    .children()
                    .find(|child| child.name() == "informaltable")
                    .is_some()
            })
        })
        .and_then(|para| {
            para.children()
                .find(|child| child.name() == "informaltable")
        })
        .and_then(|table| table.children().find(|child| child.name() == "tgroup"))
        .and_then(|tgroup| tgroup.children().find(|child| child.name() == "tbody"))
        .unwrap();

    for row in main_section.children().filter(|child| child.name() == "row") {
        let mut entries = row.children().filter(|child| child.name() == "entry");

        let name = entries.next().map(|entry| entry.text()).unwrap();
        let requires = match name.as_str() {
            "Audio" | "Video" => "\"AudioVideo\"",
            _ => "",
        };

        let init_str = format!("Category {{ kind: Kind::Main {{ requires: &[{}] }}, deprecated: true }}", requires);
        map.entry(name, &init_str);
    }

    Ok(map)
}