#[deny(missing_debug_implementations)]
#[forbid(unsafe_code)]

extern crate phf;

include!(concat!(env!("OUT_DIR"), "/map.rs"));

#[derive(Clone, Debug)]
pub struct Category {
    kind: Kind,
    deprecated: bool,
}

#[derive(Clone, Debug)]
pub enum Kind {
    Main {
        requires: &'static [&'static str],
    },
    Additional {
        child_of_all: &'static [&'static str],
        child_of_any: &'static [&'static str],
        needs_only_show_in: bool,
    }
}

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
