use sindra::log::LogPriority;

use visitor::state::State;
use visitor;

/// Abstract syntax tree visitor pipeline
pub fn pipeline<T>(ast: &T, mut state: &mut State) -> Result<(), String>
        where T: visitor::symbol::SymbolDefineVisitor +
                 visitor::type_visitor::TypeComputationVisitor {

    // define symbols
    match visitor::symbol::SymbolDefineVisitor::visit(ast, &mut state) {
        Ok(_) => {
            if state.logger.flush() == Some(LogPriority::Error) {
                return Err(format!("stopping due to previous error(s)"));
            }
        },
        Err(e) => {
            return Err(format!("fatal error during symbol definition: {}", e));
        }
    }

    // compute, check types
    match visitor::type_visitor::TypeComputationVisitor::visit(ast, &mut state) {
        Ok(_) => {
            if state.logger.flush() == Some(LogPriority::Error) {
                return Err(format!("stopping due to previous error(s)"));
            }
        },
        Err(e) => {
            return Err(format!("fatal error during type checking: {}", e));
        }
    }

    Ok(())
}
