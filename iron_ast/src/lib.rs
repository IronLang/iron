//! This crate defines the abstract syntax tree (AST) for the Iron programming language.
//!
//! The Iron AST is created in the parsing stage, and is used by the `iron_codegen_*` crates
//! to generate the appropriate machine code.

mod ast;

pub use ast::IronASTNode;
