//! Numeric traits for generic types like `Rect<T>` and `Frame<T>`.
//! `Num` accepts any primitive number; `Signed`, `Integer` and `Float`
use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[cfg(test)]
mod tests;

/// Base trait for all primitive integer and float types.
pub trait Num:
    Copy
    + Default
    + PartialEq
    + PartialOrd
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
{
    const ZERO: Self;
    const ONE: Self;
    const TWO: Self;

    /// Converts from f32. Integers round half to even and saturate at their limits.
    fn from_f32(value: f32) -> Self;

    fn to_f32(self) -> f32;

    /// Not named `max`, which would clash with `Ord::max` on integers.
    #[inline(always)]
    fn get_max(self, other: Self) -> Self {
        if self > other { self } else { other }
    }

    /// Not named `min`, which would clash with `Ord::min` on integers.
    #[inline(always)]
    fn get_min(self, other: Self) -> Self {
        if self < other { self } else { other }
    }
}

/// Numbers that can be negated: signed integers and floats.
pub trait Signed: Num + Neg<Output = Self> {}

/// Integer types only, signed or unsigned.
pub trait Integer: Num {}

/// Float types only, with the float math used by `Rect` and `Vec2`.
pub trait Float: Signed {
    const EPSILON: Self;
    const PI: Self;

    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn abs(self) -> Self;
    fn sqrt(self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn atan2(self, other: Self) -> Self;
}

macro_rules! impl_int {
    ($($t:ty),*) => {$(
        impl Num for $t {
            const ZERO: Self = 0;
            const ONE: Self = 1;
            const TWO: Self = 2;

            #[inline(always)]
            fn from_f32(value: f32) -> Self {
                // `as` saturates at the type's limits and turns NaN into 0
                value.round_ties_even() as Self
            }

            #[inline(always)]
            fn to_f32(self) -> f32 {
                self as f32
            }
        }

        impl Integer for $t {}
    )*};
}

macro_rules! impl_float {
    ($($t:ident),*) => {$(
        impl Num for $t {
            const ZERO: Self = 0.0;
            const ONE: Self = 1.0;
            const TWO: Self = 2.0;

            #[inline(always)]
            fn from_f32(value: f32) -> Self {
                value as Self
            }

            #[inline(always)]
            fn to_f32(self) -> f32 {
                self as f32
            }
        }

        impl Signed for $t {}

        impl Float for $t {
            const EPSILON: Self = $t::EPSILON;
            const PI: Self = core::$t::consts::PI;

            #[inline(always)]
            fn floor(self) -> Self {
                $t::floor(self)
            }

            #[inline(always)]
            fn ceil(self) -> Self {
                $t::ceil(self)
            }

            #[inline(always)]
            fn round(self) -> Self {
                $t::round(self)
            }

            #[inline(always)]
            fn abs(self) -> Self {
                $t::abs(self)
            }

            #[inline(always)]
            fn sqrt(self) -> Self {
                $t::sqrt(self)
            }

            #[inline(always)]
            fn powi(self, n: i32) -> Self {
                $t::powi(self, n)
            }

            #[inline(always)]
            fn sin(self) -> Self {
                $t::sin(self)
            }

            #[inline(always)]
            fn cos(self) -> Self {
                $t::cos(self)
            }

            #[inline(always)]
            fn atan2(self, other: Self) -> Self {
                $t::atan2(self, other)
            }
        }
    )*};
}

impl_int!(u8, u16, u32, u64, usize, i8, i16, i32, i64, isize);
impl_float!(f32, f64);

impl Signed for i8 {}
impl Signed for i16 {}
impl Signed for i32 {}
impl Signed for i64 {}
impl Signed for isize {}
