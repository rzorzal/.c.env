#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl { private_: bool, name: Ident, value: Expr },
    Assignment { target: Ident, value: Expr },
    ExprStmt(Expr),
}

pub type Ident = String;

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Bool(bool),
    Null,
    StringLiteral(String),
    Template(Vec<TemplatePart>),
    Ident(Ident),

    // Pós-fixos
    Call { callee: Box<Expr>, args: Vec<Expr> },
    Index { target: Box<Expr>, index: Box<Expr> },

    // Unários e binários
    Unary { op: UnaryOp, rhs: Box<Expr> },
    Binary { lhs: Box<Expr>, op: BinOp, rhs: Box<Expr> },

    // Ternário “if ... ? ... else ...”
    IfExpr { cond: Box<Expr>, then_: Box<Expr>, else_: Box<Expr> },

    // Compreensões
    ArrayComp(ArrayComp),
    FindComp(FindComp), // modo (& break)
}

#[derive(Debug, Clone)]
pub enum TemplatePart {
    Text(String),
    Expr(Expr),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp { Plus, Minus }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Or,    // |
    And,   // &
    Eq, Ne,
    Lt, Le, Gt, Ge,
    Add, Sub, Mul, Div, Mod,
}

#[derive(Debug, Clone)]
pub struct ArrayComp {
    pub expr: Box<Expr>,           // expressão a incluir
    pub var: Ident,                // variável do laço
    pub mode: IterMode,            // Of = valores, In = índices
    pub iter: Box<Expr>,           // iterável
    pub filter: Option<Box<Expr>>, // condicional
}

#[derive(Debug, Clone)]
pub struct FindComp {
    pub select: Box<Expr>,         // o (<expr> & break) sem o marcador
    pub var: Ident,
    pub mode: IterMode,
    pub iter: Box<Expr>,
    pub filter: Option<Box<Expr>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IterMode { Of, In }

// Implementation of Program
impl Program {
    pub fn new(items: Vec<Stmt>) -> Self {
        Program { items }
    }

    pub fn empty() -> Self {
        Program { items: Vec::new() }
    }
}
