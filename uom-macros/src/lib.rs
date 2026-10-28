//! Unpublished procedural macro placeholder inherited from upstream `uom`.
//!
//! The `system!` and `quantity!` macros currently return their input unchanged. The
//! [`uom-ng`](https://docs.rs/uom-ng) library defines its macros independently and does not depend
//! on this crate.

// Rustc lints.
#![forbid(unsafe_code)]
#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

#[allow(unused_extern_crates)]
extern crate proc_macro;

use proc_macro::TokenStream;

/// Define a system of quantities.
#[proc_macro]
pub fn system(input: TokenStream) -> TokenStream {
    input
}

/// Define a quantity within a system of quantities.
#[proc_macro]
pub fn quantity(input: TokenStream) -> TokenStream {
    input
}
