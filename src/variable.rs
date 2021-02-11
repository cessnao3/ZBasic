pub enum VariableType
{
    Boolean(bool),
    Integer(i32),
    Float(f32)
}

pub struct Variable
{
    pub vartype: VariableType,
    pub varname: String,
}
