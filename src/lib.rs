//! piske programming language.

#![warn(missing_docs)]

#[macro_use] extern crate lazy_static;
#[macro_use] extern crate quote;

extern crate regex;

#[macro_use] extern crate sindra;
extern crate psk_std;

pub mod ast;

pub mod parse {
    //! Parser used for the interpreter use case. Uses an annotation with a scope that includes
    //! both symbol table and memory storage.

    // allow missing docs in generated code
    #![allow(missing_docs)]
    use sindra::Identifier;

    include!(concat!(env!("OUT_DIR"), "/piske.rs"));
}

pub mod ptype;
pub use ptype::PType;

pub mod symbol;
pub use symbol::Symbol;

pub mod value;
pub mod visitor;

pub mod glue;
