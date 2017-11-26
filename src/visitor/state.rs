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
use psk_std::Environment;

/// State carried throughout the tree walker. Contains scope information and logger.
pub struct State {
    /// Reference to the current scope stack.
    pub scope: Rc<RefCell<MemoryScope<Symbol, Value>>>,
    /// Reference to the global scope.
    pub global: Rc<RefCell<MemoryScope<Symbol, Value>>>,
    /// Logger
    pub logger: LogListener<String, io::Stdout, io::Stderr>,
    /// Standard function environment
    pub std_env: Environment,
}
impl Default for State {
    fn default() -> State {
        let global = Rc::new(RefCell::new(MemoryScope::default()));
        let env = Environment::new(&mut *global.borrow_mut());

        let mut state = State {
            scope: Rc::clone(&global),
            global: global,
            logger: LogListener::new(io::stdout(), io::stderr()),
            std_env: env,
        };

        // define builtins in top-level (global) scope
        state.define_builtins();
        state
    }
}
impl State {
    /// Defines the piske built-in types and links standard library.
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
