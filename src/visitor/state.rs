//! State struct used in all visitors.

use std::io;
use std::cell::RefCell;
use std::rc::Rc;

use sindra::Identifier;
use sindra::log::LogListener;
use sindra::scope::{MemoryScope, SymbolStore};

use Symbol;
use PType;
use value::Value;

/// State carried throughout the tree walker. Contains scope information and logger.
pub struct State {
    /// Reference to the current scope stack.
    pub scope: Rc<RefCell<MemoryScope<Symbol, Value>>>,
    /// Reference to the global scope.
    pub global: Rc<RefCell<MemoryScope<Symbol, Value>>>,
    /// Logger
    pub logger: LogListener<String, io::Stdout, io::Stderr>,
}
impl Default for State {
    fn default() -> State {
        let global = Rc::new(RefCell::new(MemoryScope::default()));
        State {
            scope: Rc::clone(&global),
            global: global,
            logger: LogListener::new(io::stdout(), io::stderr()),
        }
    }
}
impl State {
    /// Defines the piske built-in types.
    pub fn define_builtins(&mut self) {
        let mut sc = self.scope.borrow_mut();
        sc.define(Identifier("int".to_string()), Symbol::builtin(Identifier("int".to_string()),
            PType::Int));
        sc.define(Identifier("float".to_string()), Symbol::builtin(Identifier("float".to_string()),
            PType::Float));
        sc.define(Identifier("bool".to_string()), Symbol::builtin(Identifier("bool".to_string()),
            PType::Boolean));
        sc.define(Identifier("complex".to_string()),
            Symbol::builtin(Identifier("complex".to_string()), PType::Boolean));
        sc.define(Identifier("string".to_string()),
            Symbol::builtin(Identifier("string".to_string()), PType::String));
    }
}
