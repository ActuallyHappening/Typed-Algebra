use std::str::FromStr;

use syn::Token;

use crate::equation::Equation;

/// Syntactically OK mathematical statement
enum MathStatement {
	Equation(Equation)
}

type Equals = Token![=];
type Plus = Token![+];
type Minus = Token![-];
type Star = Token![*];
type Slash = Token![/];
type Caret = Token![^];

impl FromStr for crate::equation::Expression {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		unimplemented!()
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn tokenize() {
		
	}
}