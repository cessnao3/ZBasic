use crate::token::{Token, TokenList};

pub struct OperationNumericExpression
{
    pub op: NumericOpType,
    pub a: Box<NumericExpression>,
    pub b: Box<NumericExpression>
}

pub enum NumericExpressionType
{
    Operation(OperationNumericExpression),
    IntConstant(i32),
    FloatConstant(f32)
}

pub enum NumericExpressionDataType
{
    Float,
    Int
}

pub struct NumericExpression
{
    pub inverted: bool,
    pub value: NumericExpressionType,
    pub data_type: NumericExpressionDataType
}

pub enum NumericOpType
{
    Add,
    Subtract,
    Multiply,
    Divide
}

impl NumericOpType
{
    pub fn parse(tokens: &mut TokenList) -> Option<NumericOpType>
    {
        // Check to see if a token is available
        if !tokens.available()
        {
            return None;
        }

        // Peek the next token
        let next = tokens.peek().unwrap();

        // Iterate over the possible operations
        let op_type: Option<NumericOpType> = match next
        {
            Token::Operator(s) =>
                {
                    // Check if the token matches the possible operation
                    if s == "+"
                    {
                        Some(NumericOpType::Add)
                    }
                    else if s == "-"
                    {
                        Some(NumericOpType::Subtract)
                    }
                    else if s == "*"
                    {
                        Some(NumericOpType::Multiply)
                    }
                    else if s == "/"
                    {
                        Some(NumericOpType::Divide)
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
