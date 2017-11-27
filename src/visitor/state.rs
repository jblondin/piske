//! State struct used in all visitors.

use std::io::{self, Read, Write};
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
    /// Input / output
    pub io: Io,
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
            io: Io::default(),
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

/// Input / Output handling
pub struct Io {
    /// Standard environment standard input
    stdin: Box<Read>,
    /// Standard environment standard output
    stdout: Box<Write>,
    /// Standard environment standard error
    stderr: Box<Write>,
}
impl Default for Io {
    fn default() -> Io {
        Io {
            stdin: Box::new(io::stdin()),
            stdout: Box::new(io::stdout()),
            stderr: Box::new(io::stderr()),
        }
    }
}
impl Io {
    /// Change the `Read` object used for standard input
    pub fn set_stdin<R: 'static + Read>(&mut self, input: R) {
        self.stdin = Box::new(input);
    }
    /// Change the `Write` object used for standard output
    pub fn set_stdout<W: 'static + Write>(&mut self, out: W) {
        self.stdout = Box::new(out);
    }
    /// Change the `Write` object used for standard error
    pub fn set_stderr<W: 'static + Write>(&mut self, err: W) {
        self.stderr = Box::new(err);
    }

    /// Retrieve a mutable reference to the standard input.
    pub fn stdin(&mut self) -> &mut Read { self.stdin.as_mut() }
    /// Retrieve a mutable reference to the standard output.
    pub fn stdout(&mut self) -> &mut Write { self.stdout.as_mut() }
    /// Retrieve a mutable reference to the standard error.
    pub fn stderr(&mut self) -> &mut Write { self.stderr.as_mut() }
}
