use std::num::*;

/// A marker trait for numbers that are not zero.
/// Implemented for [NonZeroI128] from std::num, and similar types
pub trait NonZeroPrimInt: Eq + std::fmt::Debug {
		
}

macro_rules! impl_non_zero_prim_int {
	($($t:ty),*) => ($(
		impl NonZeroPrimInt for $t {}
	)*)
}

impl_non_zero_prim_int!(NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize);