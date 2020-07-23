//! Construction of MIR from HIR.
//!
//! This crate also contains the match exhaustiveness and usefulness checking.

#![feature(rustc_private)]

#![feature(box_patterns)]
#![feature(box_syntax)]
#![feature(const_if_match)]
#![feature(const_fn)]
#![feature(const_panic)]
#![feature(crate_visibility_modifier)]
#![feature(bool_to_option)]
#![feature(or_patterns)]
#![recursion_limit = "256"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate rustc_middle;

extern crate rustc_arena;
extern crate rustc_apfloat;
extern crate rustc_attr;
extern crate rustc_data_structures;
extern crate rustc_index;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_infer;
extern crate rustc_serialize;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;
extern crate rustc_ast;

mod build;
pub mod hair;
mod lints;

use rustc_middle::ty::query::Providers;

pub fn provide(providers: &mut Providers<'_>) {
    providers.check_match = hair::pattern::check_match;
    providers.lit_to_const = hair::constant::lit_to_const;
    providers.mir_built = build::mir_built;
}
