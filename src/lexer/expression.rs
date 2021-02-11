use crate::token::Token;

use super::boolean::BoolExpression;
use super::numeric::NumericExpression;

pub enum Expression
{
    Numeric(NumericExpression),
    Boolean(BoolExpression),
}
