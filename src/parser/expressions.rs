#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Null,
    Integer(i32),
    Number(f64),
    Boolean(bool),
    String(String),
    Identifier(String),

    Add(Box<(Expression, Expression)>),
    Sub(Box<(Expression, Expression)>),
    Mul(Box<(Expression, Expression)>),
    Div(Box<(Expression, Expression)>),
    Modulo(Box<(Expression, Expression)>),
    Power(Box<(Expression, Expression)>),
    Minus(Box<Expression>),
    Plus(Box<Expression>),

    BitwiseAnd(Box<(Expression, Expression)>),
    BitwiseOr(Box<(Expression, Expression)>),
    BitwiseXor(Box<(Expression, Expression)>),
    BitwiseNot(Box<Expression>),
    LeftShift(Box<(Expression, Expression)>),
    RightShift(Box<(Expression, Expression)>),

    And(Box<(Expression, Expression)>),
    Or(Box<(Expression, Expression)>),
    Not(Box<Expression>),

    Equal(Box<(Expression, Expression)>),
    NotEqual(Box<(Expression, Expression)>),
    LessThan(Box<(Expression, Expression)>),
    GreaterThan(Box<(Expression, Expression)>),
    LessOrEqual(Box<(Expression, Expression)>),
    GreaterOrEqual(Box<(Expression, Expression)>),

    Let(Box<LetData>),
    Var(Box<VarData>),

    Assign(Box<AssignData>),
    AddAssign(Box<AssignData>),
    SubAssign(Box<AssignData>),
    MulAssign(Box<AssignData>),
    DivAssign(Box<AssignData>),
    ModuloAssign(Box<AssignData>),
    PowerAssign(Box<AssignData>),
    BitwiseAndAssign(Box<AssignData>),
    BitwiseOrAssign(Box<AssignData>),
    BitwiseXorAssign(Box<AssignData>),
    LeftShiftAssign(Box<AssignData>),
    RightShiftAssign(Box<AssignData>),

    Function(Box<FunctionData>),
    Call(Box<CallData>),
    Return(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetData {
    pub name: String,
    pub type_: Option<String>,
    pub value: Expression,
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
    pub body: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallData {
    pub ref_: Expression,
    pub args: Vec<Expression>,
}
