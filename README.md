# freedesktop-categories

[![Build Status][s1]][cc] [![Crates.io][s2]][ci] [![Documentation][s3]][docs]

[s1]: https://circleci.com/gh/ebkalderon/freedesktop-categories.svg?style=shield
[cc]: https://circleci.com/gh/ebkalderon/freedesktop-categories
[s2]: https://img.shields.io/crates/v/freedesktop-categories.svg
[ci]: https://crates.io/crates/freedesktop-categories
[s3]: https://img.shields.io/badge/docs-master-blue.svg
[docs]: https://docs.rs/freedesktop-categories

Static hash map of all application categories as defined by the Freedesktop.org
[Desktop Menu Specification][dm].

[dm]: https://specifications.freedesktop.org/menu-spec/menu-spec-latest.html

These categories are used in the parsing of `.desktop` entries on many open
source desktop environments, among other things. They are also present in other
package metadata standards like [AppStream][as].

[as]: https://www.freedesktop.org/software/appstream/docs/index.html

## Contributing

This is a community project that welcomes contributions from anyone. If you're
interested in helping out, please check the [issue tracker][it] to get started.
Pull requests and issues are welcome!

[it]: https://github.com/ebkalderon/freedesktop-categories/issues

## License

`freedesktop-categories` is free and open source software distributed under the
terms of both the [MIT License][lm] and the [Apache License 2.0][la].

[lm]: ./LICENSE-MIT
[la]: ./LICENSE-APACHE

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
