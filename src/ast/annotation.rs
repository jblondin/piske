//! AST annotation structure and associated implementations.

use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use PType;
use Symbol;
use value::Value;

use sindra::{Type, Typed, Identifier};
use sindra::scope::{Scope, Scoped, MemoryScope, SymbolStore, MemoryStore};

/// Annotation type for piske abstract syntax tree. Contains a symbol scope, memory scope,
/// and typing information.
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    /// The scope for a particular AST node
    scope: Option<Rc<RefCell<MemoryScope<Symbol, Value>>>>,
    /// The original inferred type for this AST node
    pub ty: Option<PType>,
    /// The promoted type for this AST node
    pub promote_ty: Option<PType>,
}
impl Default for Annotation {
    fn default() -> Annotation {
        Annotation {
            scope: None,
            ty: None,
            promote_ty: None,
        }
    }
}

impl Typed<PType> for Annotation {
    fn ty(&self) -> Option<PType> { self.ty.clone() }
    fn set_type(&mut self, ty: Option<PType>) { self.ty = ty; }
    fn promote_type(&self) -> Option<PType> { self.promote_ty.clone() }
    fn set_promote_type(&mut self, ty: Option<PType>) { self.promote_ty = ty; }
}

// impl Scoped<SymbolScope<Symbol>> for Annotation {
//     fn scope(&self) -> Option<Rc<RefCell<SymbolScope<Symbol>>>> {
//         match self.sym_scope {
//             Some(ref sc) => Some(Rc::clone(&sc)),
//             None => None
//         }
//     }
//     fn set_scope(&mut self, scope: Option<Rc<RefCell<SymbolScope<Symbol>>>>) {
//         self.sym_scope = scope;
//     }
// }
impl Scoped<MemoryScope<Symbol, Value>> for Annotation {
    fn scope(&self) -> Option<Rc<RefCell<MemoryScope<Symbol, Value>>>> {
        match self.scope {
            Some(ref sc) => Some(Rc::clone(&sc)),
            None => None
        }
    }
    fn set_scope(&mut self, scope: Option<Rc<RefCell<MemoryScope<Symbol, Value>>>>) {
        self.scope = scope;
    }
}

impl SymbolStore<Symbol> for Annotation {
    fn define(&mut self, ident: Identifier, symbol: Symbol) -> Option<Symbol> {
        match self.scope {
            Some(ref mut scope) => {
                scope.borrow_mut().define(ident, symbol)
            },
            None => None
        }
    }
    fn resolve(&self, ident: &Identifier) -> Option<Symbol> {
        match self.scope {
            Some(ref scope) => {
                scope.borrow().resolve(ident)
            },
            None => None
        }
    }
}
impl MemoryStore<Value> for Annotation {
    fn set(&mut self, ident: Identifier, value: Value) -> Result<Option<Value>, String> {
        match self.scope {
            Some(ref mut scope) => {
                scope.borrow_mut().set(ident, value)
            },
            None => Err(format!("variable not found: {}", ident))
        }
    }
    fn get(&self, ident: &Identifier) -> Option<Value> {
        match self.scope {
            Some(ref scope) => {
                scope.borrow().get(ident)
            },
            None => None
        }
    }
}


impl fmt::Display for Annotation {
    fn fmt(&self, f: &mut fmt::Formatter) -> ::std::result::Result<(), fmt::Error> {
        match self.scope {
            Some(ref sc) => {
                write!(f, "scope:[")?;
                // for value in sc.borrow().item.into_iter() {
                //     write!(f, "{}", value)?;
                // }
                write!(f, "{{table}}")?;
                fn print_parents<T>(f: &mut fmt::Formatter,
                        parent: &Rc<RefCell<Scope<T>>>)
                        -> ::std::result::Result<(), fmt::Error> {
                    write!(f, "↑[")?;
                    // for value in parent.borrow().item.into_iter() {
                    //     write!(f, "{}", value)?;
                    // }
                    write!(f, "{{table}}")?;
                    if let Some(ref p) = parent.borrow().parent {
                        print_parents(f, &p)?;
                    }
                    Ok(())
                }

                if let Some(ref parent) = sc.borrow().parent {
                    print_parents(f, &parent)?;
                }
                write!(f, "]")?;

            },
            None => {
                write!(f, "scope:none")?;
            }
        };
        match self.ty {
            Some(ty) => { write!(f, " type:{}", ty.name()) },
            None => { write!(f, " type:none") },
        }
    }
}
