//! Numeric traits and implementations for the math crate.
pub mod float;
pub mod integer;
pub mod num;
pub mod signed_num;

#[cfg(test)]
pub mod tests;

// Re-export the main traits
pub use float::{Float, FloatBasic, FloatTrig};
pub use integer::Integer;
pub use num::Num;
pub use signed_num::SignedNum;

/// Implements Num for unsigned integer types
macro_rules! impl_uint_num {
    ($t:ty) => {
        impl Num for $t {
            #[inline(always)]
            fn zero() -> Self {
                0
            }

            #[inline(always)]
            fn one() -> Self {
                1
            }

            #[inline(always)]
            fn two() -> Self {
                2
            }

            #[inline(always)]
            fn four() -> Self {
                4
            }

            #[inline(always)]
            fn from_usize_checked(value: usize) -> Option<Self> {
                if value <= Self::MAX as usize { Some(value as Self) } else { None }
            }

            #[inline(always)]
            fn get_max(self, b: Self) -> Self {
                if self > b { self } else { b }
            }

            #[inline(always)]
            fn get_min(self, b: Self) -> Self {
                if self < b { self } else { b }
            }

            #[inline(always)]
            fn saturating_sub(self, rhs: Self) -> Self {
                self.saturating_sub(rhs)
            }

            #[inline(always)]
            fn saturating_add(self, rhs: Self) -> Self {
                self.saturating_add(rhs)
            }

            #[inline(always)]
            fn from_f32(value: f32) -> Self {
                if value < 0.0 {
                    0
                } else {
                    let rounded = value.round_ties_even();
                    if rounded > Self::MAX as f32 { Self::MAX } else { rounded as Self }
                }
            }

            #[inline(always)]
            fn to_f32(self) -> f32 {
                self as f32
            }
        }
    };
}

/// Implements Num for signed integer types
macro_rules! impl_sint_num {
    ($t:ty) => {
        impl Num for $t {
            #[inline(always)]
            fn zero() -> Self {
                0
            }

            #[inline(always)]
            fn one() -> Self {
                1
            }

            #[inline(always)]
            fn two() -> Self {
                2
            }

            #[inline(always)]
            fn four() -> Self {
                4
            }

            #[inline(always)]
            fn from_usize_checked(value: usize) -> Option<Self> {
                if value <= Self::MAX as usize { Some(value as Self) } else { None }
            }

            #[inline(always)]
            fn get_max(self, b: Self) -> Self {
                if self > b { self } else { b }
            }

            #[inline(always)]
            fn get_min(self, b: Self) -> Self {
                if self < b { self } else { b }
            }

            #[inline(always)]
            fn saturating_sub(self, rhs: Self) -> Self {
                self.saturating_sub(rhs)
            }

            #[inline(always)]
            fn saturating_add(self, rhs: Self) -> Self {
                self.saturating_add(rhs)
            }

            #[inline(always)]
            fn from_f32(value: f32) -> Self {
                let rounded = value.round_ties_even();
                if rounded > Self::MAX as f32 {
                    Self::MAX
                } else if rounded < Self::MIN as f32 {
                    Self::MIN
                } else {
                    rounded as Self
                }
            }

            #[inline(always)]
            fn to_f32(self) -> f32 {
                self as f32
            }
        }
    };
}

/// Implements Num for floating-point types
macro_rules! impl_float_num {
    ($t:ty) => {
        impl Num for $t {
            #[inline(always)]
            fn zero() -> Self {
                0.0
            }

            #[inline(always)]
            fn one() -> Self {
                1.0
            }

            #[inline(always)]
            fn two() -> Self {
                2.0
            }

            #[inline(always)]
            fn four() -> Self {
                4.0
            }

            #[inline(always)]
            fn from_usize_checked(value: usize) -> Option<Self> {
                Some(value as Self)
            }

            #[inline(always)]
            fn get_max(self, b: Self) -> Self {
                if self > b { self } else { b }
            }

            #[inline(always)]
            fn get_min(self, b: Self) -> Self {
                if self < b { self } else { b }
            }

            #[inline(always)]
            fn saturating_sub(self, rhs: Self) -> Self {
                self - rhs
            }

            #[inline(always)]
            fn saturating_add(self, rhs: Self) -> Self {
                self + rhs
            }

            #[inline(always)]
            fn from_f32(value: f32) -> Self {
                value as Self
            }

            #[inline(always)]
            fn to_f32(self) -> f32 {
                self as f32
            }
        }
    };
}

// Implement Num for all unsigned integer types
impl_uint_num!(u8);
impl_uint_num!(u16);
impl_uint_num!(u32);
impl_uint_num!(u64);
impl_uint_num!(usize);

// Implement Num for all signed integer types
impl_sint_num!(i8);
impl_sint_num!(i16);
impl_sint_num!(i32);
impl_sint_num!(i64);
impl_sint_num!(isize);

// Implement Num for floating point types
impl_float_num!(f32);
impl_float_num!(f64);

// Implement SignedNum for signed integer types
impl SignedNum for i8 {}
impl SignedNum for i16 {}
impl SignedNum for i32 {}
impl SignedNum for i64 {}
impl SignedNum for isize {}

// Implement SignedNum for floating point types
impl SignedNum for f32 {}
impl SignedNum for f64 {}

// Implement Integer for both signed and unsigned integer types
impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for isize {}
impl Integer for u8 {}
impl Integer for u16 {}
impl Integer for u32 {}
impl Integer for u64 {}
impl Integer for usize {}

// Implement FloatBasic trait for floating point types
impl FloatBasic for f32 {
    #[inline(always)]
    fn floor(self) -> Self {
        f32::floor(self)
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        f32::ceil(self)
    }

    #[inline(always)]
    fn round(self) -> Self {
        f32::round(self)
    }

    #[inline(always)]
    fn abs(self) -> Self {
        f32::abs(self)
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        f32::sqrt(self)
    }

    #[inline(always)]
    fn powi(self, exp: i32) -> Self {
        f32::powi(self, exp)
    }

    #[inline(always)]
    fn epsilon() -> Self {
        f32::EPSILON
    }
}

impl FloatBasic for f64 {
    #[inline(always)]
    fn floor(self) -> Self {
        f64::floor(self)
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        f64::ceil(self)
    }

    #[inline(always)]
    fn round(self) -> Self {
        f64::round(self)
    }

    #[inline(always)]
    fn abs(self) -> Self {
        f64::abs(self)
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        f64::sqrt(self)
    }

    #[inline(always)]
    fn powi(self, exp: i32) -> Self {
        f64::powi(self, exp)
    }

    #[inline(always)]
    fn epsilon() -> Self {
        f64::EPSILON
    }
}

// Implement FloatTrig trait for floating point types
impl FloatTrig for f32 {
    #[inline(always)]
    fn sin(self) -> Self {
        f32::sin(self)
    }

    #[inline(always)]
    fn cos(self) -> Self {
        f32::cos(self)
    }

    #[inline(always)]
    fn atan2(self, other: Self) -> Self {
        f32::atan2(self, other)
    }

    #[inline(always)]
    fn exp(self) -> Self {
        f32::exp(self)
    }

    #[inline(always)]
    fn pi() -> Self {
        core::f32::consts::PI
    }
}

impl FloatTrig for f64 {
    #[inline(always)]
    fn sin(self) -> Self {
        f64::sin(self)
    }

    #[inline(always)]
    fn cos(self) -> Self {
        f64::cos(self)
    }

    #[inline(always)]
    fn atan2(self, other: Self) -> Self {
        f64::atan2(self, other)
    }

    #[inline(always)]
    fn exp(self) -> Self {
        f64::exp(self)
    }

    #[inline(always)]
    fn pi() -> Self {
        core::f64::consts::PI
    }
}

// Implement Float trait for floating point types (empty, just combines both traits)
impl Float for f32 {}
impl Float for f64 {}
