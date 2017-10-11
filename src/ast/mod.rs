//! Abstract syntax tree definition and associated types and implementations.

pub mod ptype;
pub use self::ptype::PType;

pub mod annotation;
pub use self::annotation::Annotation;

/// AST annotation using a symbol table scope
pub type SymAnnotation = Annotation<::sindra::scope::SymbolScope<::sindra::Symbol<PType>>>;
/// AST annotation using a symbol table + memory space scope
pub type MemAnnotation = Annotation<::sindra::scope::MemoryScope<::sindra::Symbol<PType>,
    ::value::Value>>;

pub mod ast;
pub use self::ast::*;

pub mod display;
pub mod inference;
pub mod operator;

