//! Main abstract syntax tree specification for the piske programming language. This
//! hererogeneous node tree is annotated with an arbitrary annotation node type `A`.
//!
//! This abstract syntax tree is constructed by the rust-peg parser, and annotated by the AST
//! visitors.

use sindra::{Identifier, Node};
use ast::Annotation;

/// Root-level program. Only contains a statement block.
#[derive(Debug, Clone, PartialEq)]
pub struct Program(pub Node<Block>);
annotate!(Program, Annotation);

/// Statement block is simply a list of statements.
#[derive(Debug, Clone, PartialEq)]
pub struct Block(pub Vec<Node<Statement>>);
annotate!(Block, Annotation);

/// The various allowed statements in the piske programming language.
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// Statement only containing an expression.
    Expression(Node<Expression>),
    /// Variable declaration statement.
    Declare(Node<Identifier>, Node<Expression>),
    /// Variable assignment statement.
    Assign(Node<Identifier>, Node<Expression>),
    /// Function definition statement.
    FnDefine(FunctionDef),
    // GlobalSet(Identifier, Expression)
}
annotate!(Statement, Annotation);

/// Definition of a function.
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDef {
    /// Function name.
    pub name: Node<Identifier>,
    /// Return type.
    pub ret_type: Node<Identifier>,
    /// List of function parameters.
    pub params: Vec<Node<Parameter>>,
    /// Body of the function.
    pub body: Node<Block>,
}

/// Function parameter (used in function definitions).
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    /// Paramter variable name.
    pub name: Node<Identifier>,
    /// Parameter variable type.
    pub ty: Node<Identifier>,
}
annotate!(Parameter);

/// Valid expressions in the piske programming language.
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// A sole literal
    Literal(Node<Literal>),
    /// An identifier
    Identifier(Node<Identifier>),
    /// An infix operation, of form <expr> <op> <expr>
    Infix {
        /// The specific type of infix operation (e.g. add, subtract)
        op: InfixOp,
        /// The left operand
        left: Box<Node<Expression>>,
        /// The right operand
        right: Box<Node<Expression>>,
    },
    /// A prefix operation, of form <op> <expr>
    Prefix {
        /// The specific type of prefix operation (e.g. negation)
        op: PrefixOp,
        /// The operand
        right: Box<Node<Expression>>,
    },
    /// A postfix operation, of form <expr> <op>
    Postfix {
        /// The specific type of postfix operation
        op: PostfixOp,
        /// The operand
        left: Box<Node<Expression>>,
    },
    /// A block of statements is treated as an expression
    Block(Node<Block>),
    /// A function call
    FnCall {
        /// Function name.
        name: Node<Identifier>,
        /// List of arguments passed into the function.
        args: Vec<Node<Expression>>
    }
}
annotate!(Expression, Annotation);

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
annotate!(Literal);

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
