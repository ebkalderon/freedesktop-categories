//! XML parsing and code generation functions.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use amxml::dom::NodePtr;
use chrono::Utc;
use phf_codegen::Map;

use super::Error;

const CATEGORIES_TYPE_SIG: &str = "pub static CATEGORIES: ::phf::Map<&'static str, Category> = ";

/// Contains a builder for a `phf` static hash map.
pub struct CategoryMap(Map<String>);

impl CategoryMap {
    /// Generates a new category hash map from the given root element of the XML DOM.
    ///
    /// Returns `Ok(Self)` if the specification XML was successfully parsed and a static hash map
    /// generated, returns `Err(Error::Xml)` otherwise.
    pub fn generate(root: &NodePtr) -> Result<Self, Error> {
        let mut map = Map::new();

        parse_main_categories(&root, &mut map)?;
        parse_additional_categories(&root, &mut map)?;
        parse_reserved_categories(&root, &mut map)?;

        Ok(CategoryMap(map))
    }

    /// Writes the resulting Rust code to the file located at `out`.
    ///
    /// If the file does not exist at that location, it will automatically be created. Returns
    /// `Ok(())` if successful, returns `Err(Error::Io)` if the file could not be written.
    pub fn write_file(&self, out: &Path) -> Result<(), Error> {
        let CategoryMap(ref map) = *self;

        let mut file = BufWriter::new(File::create(&out)?);
        writeln!(
            file,
            "// Generated by the build script at {}. Do not edit directly!\n",
            Utc::now()
        )?;
        write!(file, "{}", CATEGORIES_TYPE_SIG)?;
        map.build(&mut file)?;
        write!(file, ";\n")?;

        Ok(())
    }
}

fn parse_main_categories(root: &NodePtr, map: &mut Map<String>) -> Result<(), Error> {
    let entries = root.get_nodeset(
        "//sect1[@id='main-category-registry']/para/informaltable/tgroup/tbody/row/entry",
    )?;

    let names = entries
        .chunks(3)
        .flat_map(|row| row.into_iter().flat_map(|entry| entry.nth_child(0)).nth(0))
        .map(|name| name.value());

    for name in names {
        let requires = match name.as_str() {
            "Audio" | "Video" => "\"AudioVideo\"",
            _ => "",
        };

        let init_str = format!("Category::Main {{ requires: &[{}] }}", requires);
        map.entry(name, &init_str);
    }

    Ok(())
}

fn parse_additional_categories(root: &NodePtr, map: &mut Map<String>) -> Result<(), Error> {
    let entries = root.get_nodeset(
        "//sect1[@id='additional-category-registry']/para/informaltable/tgroup/tbody/row/entry",
    )?;

    let rows = entries.chunks(3).map(|row| {
        row.into_iter()
            .flat_map(|entry| entry.nth_child(0).map(|text| text.value()))
    });

    for mut row in rows {
        let name = row.next().expect("Couldn't get name column from table row");
        let related_err: String = row.skip(1).collect();
        let related = related_err.replace("QT", "Qt"); // Fix a spelling mistake in the spec.

        let suggests = if related.contains(" or ") {
            related.split(" or ").collect()
        } else if !related.is_empty() {
            vec![related.as_str()]
        } else {
            Vec::new()
        };

        let init_str = format!(
            "Category::Additional {{ suggests: &{:?} }}",
            suggests
        );

        map.entry(name, &init_str);
    }

    Ok(())
}

fn parse_reserved_categories(root: &NodePtr, map: &mut Map<String>) -> Result<(), Error> {
    let entries = root.get_nodeset(
        "//sect1[@id='reserved-category-registry']/para/informaltable/tgroup/tbody/row/entry",
    )?;

    let names = entries
        .chunks(2)
        .flat_map(|row| row.into_iter().flat_map(|entry| entry.nth_child(0)).nth(0))
        .map(|name| name.value());

    for name in names {
        map.entry(name, "Category::Reserved");
    }

    Ok(())
}
