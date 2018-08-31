#[cfg(feature = "generate-map")]
extern crate freedesktop_categories_codegen as codegen;

fn main() {
    #[cfg(feature = "generate-map")]
    codegen::DesktopMenuSpec::new()
        .version(codegen::Version::Latest)
        .output_file_name("map.rs")
        .gen_static_map()
        .unwrap();
}
