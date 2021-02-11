/*
ZBasic Grammar

Program ->
    <>
    <Statement>

Statement ->
    <>
    <Statement> <Statement>
    if (<BoolExpr>) { <Statement> }
    if (<BoolExpr>) { <Statement> } else { <Statement> }
    while (<BoolExpr>) { <Statement> }
    Var = <Expr>;

Expr ->
    <NumExpr>
    <BoolExpr>

NumExpr ->
    Float
    Int
    -Float
    -Int
    <NumExpr> <NumericOp> <NumExpr>
    (<NumExpr>)

BoolExpr ->
    Bool
    !<BoolExpr>
    <BoolExpr> <BoolOp> <BoolExpr>
    (<BoolExpr>)

NumericOp ->
    +
    -
    /
    *

BoolOp ->
    &&
    ||
 */

mod boolean;
mod expression;
mod numeric;
mod program;
mod statement;
