use quote::Tokens;

use sindra::log::LogPriority;

use parse;
use glue::pipeline;
use visitor::{self, State};

fn transpile_pipeline<T>(ast: &T, mut state: &mut State) -> Result<Tokens, String>
        where T: visitor::symbol::SymbolDefineVisitor +
                 visitor::type_visitor::TypeComputationVisitor +
                 visitor::transpile::TranspileVisitor {
    pipeline(ast, &mut state)?;

    // transpile
    let transpiled = {
        match visitor::transpile::TranspileVisitor::visit(ast, &mut state) {
            Ok(value) => {
                if state.logger.flush() == Some(LogPriority::Error) {
                    return Err(format!("stopping due to previous error(s)"));
                }
                value
            },
            Err(e) => {
                return Err(format!("fatal error during transpilation: {}", e));
            }
        }
    };

    Ok(transpiled)
}

/// Transpile a program, given as a string.
pub fn transpile(program: &str) -> Result<Tokens, String> {
    // lex the program
    let ast = match parse::program(program) {
        Ok(ast) => ast,
        Err(e) => {
            return Err(format!("failed to lex program: {}", e));
        }
    };

    // set up a default state
    let mut state = State::default();

    transpile_pipeline(&ast, &mut state)
}
