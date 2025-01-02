use crate::token::Token;

#[derive(Debug)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Literal(LiteralValue),
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}

impl Expr {
    pub fn print(&self) -> String {
        match self {
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                format!("({} {} {})", operator.lexeme, left.print(), right.print())
            }
            Expr::Grouping(expr) => format!("(group {})", expr.print()),
            Expr::Literal(v) => match v {
                LiteralValue::Nil => "nil".to_string(),
                _ => format!("{:?}", v),
            },
            Expr::Unary { operator, right } => format!("({} {})", operator.lexeme, right.print()),
        }
    }
}
