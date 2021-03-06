use std::fmt::{Debug, Formatter};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: String,
    pub vars: Vec<Var>,
    pub exprs: Vec<Expr>,
    pub ret: i32,
}
#[derive(Clone)]
pub struct Var {
    pub(crate) name: String,
    pub(crate) expr: Expr,
}

impl Debug for Var {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.expr {
            Expr::Unary { child, op } => {
                write!(f, "let {} = {:?}{:?}", &self.name, child, op)
            }
            Expr::Binary { lhs, op, rhs } => {
                write!(f, "let {} = {:?} {:?} {:?}", &self.name, lhs, op, rhs)
            }
            Expr::Literal(s) => {
                write!(f, "let {} = {}", &self.name, s)
            }
            Expr::Reference(s) => {
                write!(f, "let {} = {}", &self.name, s)
            }
        }
    }
}

#[derive(Clone)]
pub enum Expr {
    Unary {
        child: Box<Expr>,
        op: Operator,
    },
    // { operator - term }
    Binary {
        lhs: Box<Expr>,
        op: Operator,
        rhs: Box<Expr>,
    },
    // { term - {operator - term)* }
    Literal(i32),
    Reference(String),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Unary { child, op } => match op {
                Operator::Add => {
                    write!(f, "{:?}", child)
                }
                Operator::Sub => {
                    write!(f, "-{:?}", child)
                }
                _ => unimplemented!(),
            },
            Expr::Binary { lhs, op, rhs } => match op {
                Operator::Incr => {
                    write!(f, "{:?}", lhs)
                }
                Operator::Decr => {
                    write!(f, "{:?}--", lhs)
                }
                Operator::Comp => {
                    write!(f, "!{:?}", lhs)
                }
                _ => {
                    write!(f, "{:?} {:?} {:?}", lhs, op, rhs)
                }
            },
            Expr::Literal(i) => {
                write!(f, "{}", i)
            }
            Expr::Reference(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

#[derive(Clone)]
pub enum Operator {
    Add,
    // addition         | +     | binary | unary
    Sub,
    // subtraction      | -     | binary | unary
    Mul,
    // division         | *     | binary
    Div,
    // multiplication   | /     | binary
    Incr,
    // increase         | ++    | crement
    Decr,
    // decrease         | --    | crement
    Comp,
    // complement       | !     | unary
}

impl Debug for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operator::Add => {
                    "+"
                }
                Operator::Sub => {
                    "-"
                }
                Operator::Mul => {
                    "*"
                }
                Operator::Div => {
                    "/"
                }
                Operator::Incr => {
                    "++"
                }
                Operator::Decr => {
                    "--"
                }
                Operator::Comp => {
                    "!"
                }
            }
        )
    }
}
