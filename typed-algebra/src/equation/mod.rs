use crate::{fraction::FractionI32, pronumeral::Pronumeral};

pub mod quadratic;

pub struct Equation {

}

pub enum Expression {
	Literal(FractionI32),
	Pronumeral(Pronumeral),
	UnaryOperation(Operator, Box<Expression>),
	BinaryOperation(Box<Expression>, Operator, Box<Expression>),
}

pub enum Operator {
	Plus,
	Minus,
	Multiply,
	Divide,
}