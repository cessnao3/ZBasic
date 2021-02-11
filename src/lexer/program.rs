use super::statement::Statement;
use crate::variable::Variable;
use std::collections::HashMap;

/// Defines the overall program
pub struct Program
{
    /// Provides the main/first program
    pub main: Statement,

    /// Defines the variables
    pub variables: HashMap<String, Variable>
}
