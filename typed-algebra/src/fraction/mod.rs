mod new;
pub use new::*;

#[derive(Debug)]
pub struct Fraction<N: Eq + std::fmt::Debug, D: Eq + std::fmt::Debug> {
	numerator: N,
	denominator: D,
}