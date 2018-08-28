//! Parser and code generator for `freedesktop-categories`.
//!
//! Fetches the latest DocBook version of the [Desktop Menu Specification][dm] from
//! Freedesktop.org, parses the XML, builds a static `phf` hash map of all the categories, and
//! saves the generated Rust code to a file which can be included in your Rust project during a
//! Cargo build.
//!
//! [dm]: https://specifications.freedesktop.org/menu-spec/menu-spec-1.0.html

#![deny(missing_debug_implementations)]
#![forbid(unsafe_code)]

extern crate amxml;
extern crate curl;
extern crate phf_codegen;

pub use error::Error;

use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use amxml::dom::new_document;
use curl::easy::Easy;

use generate::CategoryMap;

mod error;
mod generate;

const SPEC_URL: &str = "https://specifications.freedesktop.org/menu-spec/menu-spec-latest.xml";
const SPEC_NAME: &str = "menu-spec-latest.xml";

/// Specification parser and code generator.
#[derive(Debug)]
pub struct DesktopMenuSpec {
    xml_cache_dir: Option<PathBuf>,
    always_download: bool,
    output_name: &'static str,
}

impl DesktopMenuSpec {
    /// Creates a new code generator.
    pub fn new() -> Self {
        DesktopMenuSpec {
            xml_cache_dir: None,
            always_download: false,
            output_name: "map.rs",
        }
    }

    /// Overrides the path where the XML file should be downloaded and cached.
    ///
    /// This value is `$OUT_DIR` by default.
    pub fn xml_cache_dir<P: Into<PathBuf>>(&mut self, path: P) -> &mut Self {
        self.xml_cache_dir = Some(path.into());
        self
    }

    /// Always download the XML file again, even if it is already present in the cache.
    ///
    /// This value is `false` by default.
    pub fn always_download(&mut self, value: bool) -> &mut Self {
        self.always_download = value;
        self
    }

    /// Sets the name of the Rust output file. This name should include a `.rs` extension suffix.
    ///
    /// This value is `map.rs` by default.
    pub fn output_file_name(&mut self, name: &'static str) -> &mut Self {
        self.output_name = name;
        self
    }

    /// Generates a static hash map of application categories and saves it to a file.
    ///
    /// Returns `Ok(())` if successful, returns `Err(Error)` otherwise.
    pub fn gen_static_map(&self) -> Result<(), Error> {
        let cache_dir = self
            .xml_cache_dir
            .clone()
            .unwrap_or(env::var("OUT_DIR")?.into());

        // Remove the DocBook-specific symbols so the XML can be parsed normally.
        let xml = fetch_or_download(&cache_dir, self.always_download)?
            .replace("&version", "version")
            .replace("&dtd-version", "dtd-version");

        let doc = new_document(&xml)?;
        let root = doc.root_element();

        let map = CategoryMap::generate(&root)?;
        let out = Path::new(&env::var("OUT_DIR")?).join(self.output_name);
        map.write_file(&out)?;

        Ok(())
    }
}

fn fetch_or_download(out_dir: &Path, always_download: bool) -> Result<String, Error> {
    let path = Path::new(&out_dir).join(SPEC_NAME);

    if !path.exists() || always_download {
        let mut file = File::create(&path)?;
        let mut handle = Easy::new();
        handle.url(SPEC_URL)?;

        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            file.write(data).unwrap();
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    fs::read_to_string(&path).map_err(|e| e.into())
}
