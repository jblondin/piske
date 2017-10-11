//! AST annotation structure and associated implementations.

use std::fmt;
use std::cell::RefCell;
use std::rc::Rc;

use ast::PType;

use sindra::{Type, Typed, Identifier};
use sindra::scope::{Scope, Scoped, SymbolStore, MemoryStore};

/// Annotation type for piske abstract syntax tree. Contains a scope `Sc`, and typing information.
#[derive(Debug, Clone, PartialEq)]
pub struct Annotation<Sc> {
    /// The scope for a particular AST node, of type `Sc`
    pub scope: Option<Rc<RefCell<Sc>>>,
    /// The original inferred type for this AST node
    pub ty: Option<PType>,
    /// The promoted type for this AST node
    pub promote_ty: Option<PType>,
}
impl<Sc> Default for Annotation<Sc> {
    fn default() -> Annotation<Sc> {
        Annotation {
            scope: None,
            ty: None,
            promote_ty: None,
        }
    }
}

impl<Sc> Typed<PType> for Annotation<Sc> {
    fn ty(&self) -> Option<PType> { self.ty.clone() }
    fn set_type(&mut self, ty: Option<PType>) { self.ty = ty; }
    fn promote_type(&self) -> Option<PType> { self.promote_ty.clone() }
    fn set_promote_type(&mut self, ty: Option<PType>) { self.promote_ty = ty; }
}

impl<Sc: Default> Scoped for Annotation<Sc> {
    type Scope = Sc;
    fn scope(&self) -> Option<Rc<RefCell<Sc>>> {
        match self.scope {
            Some(ref sc) => Some(Rc::clone(&sc)),
            None => None
        }
    }
    fn set_scope(&mut self, scope: Option<Rc<RefCell<Sc>>>) {
        self.scope = scope;
    }
}

impl<Sym: Clone, Sc: SymbolStore<Sym>> SymbolStore<Sym> for Annotation<Sc> {
    fn define(&mut self, ident: Identifier, symbol: Sym) -> Option<Sym> {
        match self.scope {
            Some(ref mut scope) => {
                scope.borrow_mut().define(ident, symbol)
            },
            None => None
        }
    }
    fn resolve(&self, ident: &Identifier) -> Option<Sym> {
        match self.scope {
            Some(ref scope) => {
                scope.borrow().resolve(ident)
            },
            None => None
        }
    }
}
impl<Val, Sc> MemoryStore<Val> for Annotation<Sc> where Sc: MemoryStore<Val> {
    fn set(&mut self, ident: Identifier, value: Val) -> Result<Option<Val>, String> {
        match self.scope {
            Some(ref mut scope) => {
                scope.borrow_mut().set(ident, value)
            },
            None => Err(format!("variable not found: {}", ident))
        }
    }
    fn get(&self, ident: &Identifier) -> Option<Val> {
        match self.scope {
            Some(ref scope) => {
                scope.borrow().get(ident)
            },
            None => None
        }
    }
}


impl<T> fmt::Display for Annotation<Scope<T>> {
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
