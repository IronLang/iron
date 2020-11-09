pub struct Parameter {
    identifier: String,
    parameter_type: String,
}

pub enum Value {
    Boolean(bool),
    Int(isize),
}
use super::op::Op;

pub enum Expression {
    Empty,
    UnaryOp {
        op: Op,
        expr: Box<Expression>,
    },
    BinaryOp {
        op: Op,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
}
