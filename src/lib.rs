//! Static hash map of all application categories defined by the Freedesktop.org
//! [Desktop Menu Specification][dm].
//!
//! [dm]: https://specifications.freedesktop.org/menu-spec/menu-spec-latest.html
//!
//! These categories are used in the parsing of `.desktop` entries on many open source desktop
//! environments, among other things. They are also present in other package metadata standards
//! like [AppStream][as].
//!
//! [as]: https://www.freedesktop.org/software/appstream/docs/index.html

#![deny(missing_debug_implementations)]
#![forbid(unsafe_code)]

extern crate phf;

#[cfg(feature = "generate-map")]
include!(concat!(env!("OUT_DIR"), "/map.rs"));

#[cfg(not(feature = "generate-map"))]
include!("map.rs");

// TODO: Hide away this type and expose something more type-safe.
/// Entry in the category hash map.
#[derive(Clone, Debug)]
pub enum Category {
    Main {
        /// Other main categories required to be used alongside this category.
        requires: &'static [&'static str],
    },
    Additional {
        /// Related categories which are suggested to be used alongside this one.
        suggests: &'static [&'static str],
    },
    /// Reserved tags require users to specify the `OnlyShowIn` tag in entries.
    Reserved,
}

/// Returns whether the given string is a valid category.
///
/// This function does not differentiate between different kinds of categories. Semicolon-delimited
/// category sets such as `Foo;Bar;Baz` are not counted by this function.
///
/// # Example
///
/// ```
/// # use freedesktop_categories::is_category;
/// assert!(is_category("AudioVideo"));
/// assert!(is_category("Office"));
/// assert!(is_category("System"));
/// ```
pub fn is_category(name: &str) -> bool {
    CATEGORIES.contains_key(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn main_categories_exist() {
        assert!(is_category("AudioVideo"));
        assert!(is_category("Audio"));
        assert!(is_category("Video"));
        assert!(is_category("Development"));
        assert!(is_category("Education"));
        assert!(is_category("Game"));
        assert!(is_category("Graphics"));
        assert!(is_category("Network"));
        assert!(is_category("Office"));
        assert!(is_category("Settings"));
        assert!(is_category("System"));
        assert!(is_category("Utility"));
    }
}
