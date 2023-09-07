mod new;
pub use new::*;

#[derive(Debug)]
pub struct Fraction<N: Eq + std::fmt::Debug, D: Eq + std::fmt::Debug> {
	pub numerator: N,
	pub denominator: D,
}

pub type FractionI32 = Fraction<i32, std::num::NonZeroI32>;