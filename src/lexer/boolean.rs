use crate::token::{Token, TokenList};

pub struct OperationBoolExpression
{
    op: BooleanOpType,
    a: Box<BoolExpression>,
    b: Box<BoolExpression>
}

pub enum BoolExpressionType
{
    Operation(OperationBoolExpression),
    Constant(bool)
}

pub struct BoolExpression
{
    inverted: bool,
    value: BoolExpressionType
}

pub enum BooleanOpType
{
    And,
    Or
}

impl BooleanOpType
{
    pub fn parse(tokens: &mut TokenList) -> Option<BooleanOpType>
    {
        // Check to see if a token is available
        if !tokens.available()
        {
            return None;
        }

        // Peek the next token
        let next = tokens.peek().unwrap();

        // Iterate over the possible operations
        let op_type: Option<BooleanOpType> = match next
        {
            Token::Operator(s) =>
                {
                    // Check if the token matches the possible operation
                    if s == "&&"
                    {
                        Some(BooleanOpType::And)
                    }
                    else if s == "||"
                    {
                        Some(BooleanOpType::Or)
                    }
                    else
                    {
                        None
                    }
                },
            _ => None
        };

        // Return the result
        return match op_type
        {
            None => None,
            Some(op) =>
                {
                    // Pop the current token
                    tokens.pop();

                    // Return the result
                    Some(op)
                }
        }
    }
}
