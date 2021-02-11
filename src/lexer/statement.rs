use crate::token::TokenList;

use super::expression::Expression;
use super::boolean::BoolExpression;

/// Defines a statement program
pub struct Statement
{
    /// Defines the tokens within the statement
    pub data: StatementType,

    /// Defines the optional next statement
    pub next: Option<Box<Statement>>
}

pub enum StatementType
{
    If(IfStatement),
    While(WhileStatement),
    Expr(Expression)
}

pub struct IfStatement
{
    pub boolexpr: BoolExpression,
    pub statement: Box<Statement>,
    pub else_statement: Box<Statement>
}

pub struct WhileStatement
{
    pub boolexpr: BoolExpression,
    pub statement: Box<Statement>
}

pub struct VarStatement
{
    pub varname: String,
    pub expr: Expression
}
