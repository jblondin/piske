#[macro_export]
macro_rules! expr_stmt {
    ($expr:expr) => ( Node::new(Statement::Expression($expr)) )
}
#[macro_export]
macro_rules! decl_stmt {
    ($ident:expr, $expr:expr) => ( Node::new(Statement::Declare($ident, $expr)) )
}
#[macro_export]
macro_rules! assign_stmt {
    ($ident:expr, $expr:expr) => ( Node::new(Statement::Assign($ident, $expr)) )
}

#[macro_export]
macro_rules! ident_expr {
    ($ident:expr) => ( Node::new(Expression::Identifier($ident)) )
}
#[macro_export]
macro_rules! ident {
    ($ident:ident) => ( Node::new(Identifier(stringify!($ident).to_string())) )
}
#[macro_export]
macro_rules! literal {
    ($lit:expr) => ( Node::new(Expression::Literal($lit))  )
}
#[macro_export]
macro_rules! int { ($value:expr) => ( literal!(Node::new(Literal::Int($value))) ) }

#[macro_export]
macro_rules! infix {
    ($op:path, $left:expr, $right:expr) => (
        Node::new(Expression::Infix {
            op: $op,
            left: Box::new($left),
            right: Box::new($right)
        })
    )
}
#[macro_export]
macro_rules! add { ($left:expr, $right:expr) => ( infix!(InfixOp::Add, $left, $right) ) }
#[macro_export]
macro_rules! subtract { ($left:expr, $right:expr) => ( infix!(InfixOp::Subtract, $left, $right) ) }
#[macro_export]
macro_rules! multiply { ($left:expr, $right:expr) => ( infix!(InfixOp::Multiply, $left, $right) ) }
#[macro_export]
macro_rules! divide { ($left:expr, $right:expr) => ( infix!(InfixOp::Divide, $left, $right) ) }

#[macro_export]
macro_rules! prefix {
    ($op:path, $right:expr) => (
        Node::new(Expression::Prefix {
            op: $op,
            right: Box::new($right)
        })
    )
}
#[macro_export]
macro_rules! uminus { ($value:expr) => ( prefix!(PrefixOp::UnaryMinus, $value) ) }
#[macro_export]
macro_rules! uplus { ($value:expr) => ( prefix!(PrefixOp::UnaryPlus, $value) ) }

#[macro_export]
macro_rules! postfix {
    ($op:path, $left:expr) => (
        Node::new(Expression::Postfix {
            op: $op,
            left: Box::new($left)
        })
    )
}
#[macro_export]
macro_rules! conj { ($value:expr) => ( postfix!(PostfixOp::Conjugate, $value) ) }
