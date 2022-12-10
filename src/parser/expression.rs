#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Null,
    Integer(i32),

    Identifier(String),

    Add(Box<(Expression, Expression)>),
    Sub(Box<(Expression, Expression)>),
    Mul(Box<(Expression, Expression)>),

    Var(Box<VarData>),

    Assign(Box<AssignData>),

    Function(Box<FunctionData>),
    Call(Box<CallData>),
    Return(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarData {
    pub name: String,
    pub type_: Option<String>,
    pub value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssignData {
    pub ref_: Expression,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionData {
    pub name: Option<String>,
    pub args: Vec<(String, String)>,
    pub return_type: Option<String>,
    pub body: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallData {
    pub ref_: Expression,
    pub args: Vec<Expression>,
}
