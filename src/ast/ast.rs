//! Main abstract syntax tree specification for the piske programming language. This
//! hererogeneous node tree is annotated with an arbitrary annotation node type `A`.
//!
//! This abstract syntax tree is constructed by the rust-peg parser, and annotated by the AST
//! visitors.

use sindra::{Identifier, Node};

/// Root-level program. Only contains a statement block.
#[derive(Debug, Clone, PartialEq)]
pub struct Program<A: Default>(pub Node<Block<A>, A>);

/// Statement block is simply a list of statements.
#[derive(Debug, Clone, PartialEq)]
pub struct Block<A: Default>(pub Vec<Node<Statement<A>, A>>);

/// The various allowed statements in the piske programming language.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement<A: Default> {
    /// Statement only containing an expression.
    Expression(Node<Expression<A>, A>),
    /// Variable declaration statement.
    Declare(Node<Identifier, A>, Node<Expression<A>, A>),
    /// Variable assignment statement
    Assign(Node<Identifier, A>, Node<Expression<A>, A>),
    // GlobalSet(Identifier, Expression)
}

/// Valid expressions in the piske programming language.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression<A: Default> {
    /// A sole literal
    Literal(Node<Literal, A>),
    /// An identifier
    Identifier(Node<Identifier, A>),
    /// An infix operation, of form <expr> <op> <expr>
    Infix {
        /// The specific type of infix operation (e.g. add, subtract)
        op: InfixOp,
        /// The left operand
        left: Box<Node<Expression<A>, A>>,
        /// The right operand
        right: Box<Node<Expression<A>, A>>,
    },
    /// A prefix operation, of form <op> <expr>
    Prefix {
        /// The specific type of prefix operation (e.g. negation)
        op: PrefixOp,
        /// The operand
        right: Box<Node<Expression<A>, A>>,
    },
    /// A postfix operation, of form <expr> <op>
    Postfix {
        /// The specific type of postfix operation
        op: PostfixOp,
        /// The operand
        left: Box<Node<Expression<A>, A>>,
    },
    /// A block of statements is treated as an expression
    Block(Node<Block<A>, A>),
}

/// Supported literals
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Simple string literal
    String(String),
    /// Floating point literal
    Float(f64),
    /// Integer literal
    Int(i64),
}

/// Valid prefix operations
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PrefixOp {
    /// Unary minus (negation)
    UnaryMinus,
    /// Unary plus (posation - basically a no-op)
    UnaryPlus,
}

/// Valid infix operations
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InfixOp {
    /// Addition
    Add,
    /// Subtraction
    Subtract,
    /// Multiplication
    Multiply,
    /// Division
    Divide,
    /// Exponentiation
    Power
}

/// Valid postfix operations
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PostfixOp {
    /// Conjugation
    Conjugate,
}
