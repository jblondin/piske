//! Collection of functions for interpreting programs.

use sindra::log::LogPriority;

use visitor::{self, State};
use value::Value;
use glue::pipeline;
use parse;

/// Full interpreter pipeline
pub fn interpret_pipeline<T>(ast: &T, mut state: &mut State) -> Result<Value, String>
        where T: visitor::symbol::SymbolDefineVisitor +
                 visitor::type_visitor::TypeComputationVisitor +
                 visitor::eval::EvaluateVisitor {
    pipeline(ast, &mut state)?;

    // evaluate
    let final_val = {
        match visitor::eval::EvaluateVisitor::visit(ast, &mut state) {
            Ok(value) => {
                if state.logger.flush() == Some(LogPriority::Error) {
                    return Err(format!("stopping due to previous error(s)"));
                }
                value
            },
            Err(e) => {
                return Err(format!("fatal error during evaluation: {}", e));
            }
        }
    };

    Ok(final_val)
}

/// Interpret a single statement
pub fn interpret_statement(line: &str, mut state: &mut State)
        -> Result<Value, String> {
    // lex the statement
    let statement_ast = match parse::statement(line) {
        Ok(ast) => ast,
        Err(e) => {
            return Err(format!("failed to parse statement: {}", e));
        }
    };

    interpret_pipeline(&statement_ast, &mut state)
}

/// Interpret a program, given as a string.
pub fn interpret(program: &str) -> Result<Value, String> {
    // lex the program
    let ast = match parse::program(program) {
        Ok(ast) => ast,
        Err(e) => {
            return Err(format!("failed to lex program: {}", e));
        }
    };

    // set up a default state
    let mut state = State::default();

    interpret_pipeline(&ast, &mut state)
}
