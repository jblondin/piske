//! Abstract syntax tree definition and associated types and implementations.

pub mod annotation;
pub use self::annotation::Annotation;

// use Symbol;

// /// AST annotation using a symbol table scope
// pub type SymAnnotation = Annotation<::sindra::scope::SymbolScope<Symbol>>;
// /// AST annotation using a symbol table + memory space scope
// pub type MemAnnotation = Annotation<::sindra::scope::MemoryScope<Symbol, ::value::Value>>;

pub mod ast;
pub use self::ast::*;

pub mod display;
pub mod inference;
pub mod operator;

