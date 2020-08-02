//! This crate provides a procedural macro to set up tests based on
//! a directory of inputs to a function.
//!
//! A more complete explanation is provided in the readme of the
//! repository.
//!
//! # Overview
//!
//! The annotation has 3 primary requirements:
//!
//! * It must be annotating a function with return type implementing
//!   [`std::fmt::Debug`]. `Result` is weakly recommended.
//!
//! * It must use a string literal containing the path (relative to
//!   cargo manifest) to a base folder. A base folder must contain
//!   test folders or base folders. Test folders must contain
//!   exactly one of `input.rs`, `input.txt`, or `input.bin`.
//!
//! * It must have a single parameter of the corresponding to a
//!   respective type of the input files as included by their
//!   respective macros, `include`, `include_str`, and
//!   `include_bytes`.
//!
//! [`std::fmt::Debug`]: https://doc.rust-lang.org/std/fmt/trait.Debug.html
//!
//! # Example
//!
//! `project_root/src/lib/tests.rs`
//!
//! ```ignore
//! #[fn_fixture::snapshot("snapshot-tests/examples")]
//! fn parse_unsigned_number(value: &str) -> Result<usize, impl std::fmt::Debug> {
//!     value.parse()
//! }
//! ```
//!
//! `project_root/snapshot-tests/examples/good_number/input.txt`
//!
//! ```ignore
//! 42
//! ```
//!
//! Notice that `snapshot-tests/examples` does not itself contain an `input.txt`
//!

extern crate proc_macro;

/// Denotes the entrance point of a function-fixture's snapshots.
#[proc_macro_attribute]
pub fn snapshot(path_attr: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match fn_fixture_lib::make_snapshots(
        &path_attr.into(),
        &item.into(),
    ) {
        Ok(value) => value,
        Err(value) => value,
    }.into()
}
