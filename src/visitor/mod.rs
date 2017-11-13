//! Abstract syntax tree visitors. These submodules implement the abstract syntax tree walkers which
//! can inspect, annotate, and process the abstract syntax tree.

pub mod eval;
pub mod symbol;
pub mod type_visitor;

pub mod state;
pub use self::state::State;

// pub trait Visitor<Sc> {
//     fn visit(&mut self, state: &mut State<Sc>) -> Result;
// }
