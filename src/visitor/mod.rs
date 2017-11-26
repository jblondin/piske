//! Abstract syntax tree visitors. These submodules implement the abstract syntax tree walkers which
//! can inspect, annotate, and process the abstract syntax tree.

pub mod eval;
pub use self::eval::EvaluateVisitor;
pub mod symbol;
pub use self::symbol::SymbolDefineVisitor;
pub mod type_visitor;
pub use self::type_visitor::TypeComputationVisitor;

pub mod state;
pub use self::state::State;
