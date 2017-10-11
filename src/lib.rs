//! piske programming language.

#![warn(missing_docs)]

#[macro_use] extern crate lazy_static;

extern crate regex;
extern crate sindra;

pub mod ast;

pub mod interp_parse {
    //! Parser used for the interpreter use case. Uses an annotation with a scope that includes
    //! both symbol table and memory storage.

    // allow missing docs in generated code
    #![allow(missing_docs)]
    use sindra::Identifier;
    type Annotation = MemAnnotation;

    include!(concat!(env!("OUT_DIR"), "/piske.rs"));
}

pub mod compiler_parse {
    //! Parser used for the compiler use case. Uses an annotation with a scope that only includes
    //! symbol table.

    // allow missing docs in generated code
    #![allow(missing_docs)]
    use sindra::Identifier;
    type Annotation = SymAnnotation;

    include!(concat!(env!("OUT_DIR"), "/piske.rs"));
}

pub mod value;
pub mod visitor;
