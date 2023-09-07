use std::str::FromStr;
use num::PrimInt;

use super::Fraction;

impl<N: Eq + std::fmt::Debug, D: Eq + std::fmt::Debug> Fraction<N, D> {
    /// Creates a fully generic fraction.
    /// Use with caution.
    pub fn new_raw(numerator: N, denominator: D) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

impl<N, D> FromStr for Fraction<N, D>
where
    N: PrimInt + Eq + std::fmt::Debug + FromStr,
		D: Eq + std::fmt::Debug + FromStr,
{
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('/');
        let numerator: N = split
            .next()
            .ok_or("No numerator")?
            .trim()
            .parse::<N>()
            .map_err(|_| "Numerator is not an integer")?;
            
        let denominator: D = split
            .next()
            .ok_or("No denominator")?
            .trim()
            .parse::<D>()
            .map_err(|_| "Denominator is not an integer")?;
        if split.next().is_some() {
            return Err("Too many slashes");
        }
        Ok(Self::new_raw(numerator, denominator))
    }
}

#[cfg(test)]
mod tests {
    use std::num::*;

    use super::*;

    #[test]
    fn parses_integer() {
        let frac: Fraction<i32, NonZeroUsize> = "69 / 42".parse().unwrap();
        assert_eq!(frac.numerator, 69);
        assert_eq!(frac.denominator.get(), 42);

        "-69 / 0".parse::<Fraction<i64, NonZeroI64>>().unwrap_err();
    }
}
