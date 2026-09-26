use crate::{
    num::{Float, Num, Signed},
    vec2::Vec2,
};
use core::ops::{Add, Mul, Sub};

mod convert;
mod ops;

/// A generic rectangular area.
#[derive(Clone, Copy, Debug, Default)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T> Rect<T>
where
    T: Num,
{
    pub fn new(x: T, y: T, w: T, h: T) -> Self {
        Rect { x, y, w, h }
    }

    pub fn offset(self, delta_x: T, delta_y: T) -> Self {
        Rect {
            x: self.x + delta_x,
            y: self.y + delta_y,
            ..self
        }
    }

    pub fn pos(&self) -> Vec2<T> {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn size(&self) -> Vec2<T> {
        Vec2 {
            x: self.w,
            y: self.h,
        }
    }

    pub fn left(self) -> T {
        self.x
    }

    pub fn right(&self) -> T {
        self.x + self.w
    }

    pub fn top(&self) -> T {
        self.y
    }

    pub fn bottom(&self) -> T {
        self.y + self.h
    }

    pub fn top_left(&self) -> Vec2<T> {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }

    pub fn top_right(&self) -> Vec2<T> {
        Vec2 {
            x: self.right(),
            y: self.y,
        }
    }

    pub fn bottom_left(&self) -> Vec2<T> {
        Vec2 {
            x: self.x,
            y: self.bottom(),
        }
    }

    pub fn bottom_right(&self) -> Vec2<T> {
        Vec2 {
            x: self.x + self.w,
            y: self.y + self.h,
        }
    }

    pub fn bottom_center(&self) -> Vec2<T> {
        Vec2 {
            x: self.x + self.w / T::TWO,
            y: self.y + self.h,
        }
    }

    pub fn right_center(&self) -> Vec2<T> {
        Vec2 {
            x: self.x + self.w,
            y: self.y + self.h / T::TWO,
        }
    }

    pub fn center(&self) -> Vec2<T> {
        Vec2 {
            x: self.x + self.w / T::TWO,
            y: self.y + self.h / T::TWO,
        }
    }

    pub fn intersect(&self, other: Self) -> Option<Self> {
        // Basic overlap check using same logic as overlaps() method (inclusive boundaries)
        if other.x > self.x + self.w {
            return None;
        }
        if other.y > self.y + self.h {
            return None;
        }
        if other.x + other.w < self.x {
            return None;
        }
        if other.y + other.h < self.y {
            return None;
        }

        let x = if self.x > other.x { self.x } else { other.x };
        let y = if self.y > other.y { self.y } else { other.y };
        let right = if self.right() < other.right() {
            self.right()
        } else {
            other.right()
        };
        let bottom = if self.bottom() < other.bottom() {
            self.bottom()
        } else {
            other.bottom()
        };
        Some(Rect {
            x,
            y,
            w: right - x,
            h: bottom - y,
        })
    }

    pub fn clamp_vec(self, vec: Vec2<T>) -> Vec2<T> {
        Vec2 {
            x: if vec.x < self.x {
                self.x
            } else if vec.x > self.right() {
                self.right()
            } else {
                vec.x
            },
            y: if vec.y < self.y {
                self.y
            } else if vec.y > self.bottom() {
                self.bottom()
            } else {
                vec.y
            },
        }
    }

    pub fn shrink(self, margin: T) -> Self {
        if margin == T::ZERO {
            return self;
        }
        Self {
            x: self.x + margin,
            y: self.y + margin,
            w: T::get_max(self.w - (margin * T::TWO), T::ZERO),
            h: T::get_max(self.h - (margin * T::TWO), T::ZERO),
        }
    }

    pub fn expand(self, margin: T) -> Self {
        if margin == T::ZERO {
            return self;
        }
        Self {
            x: self.x - margin,
            y: self.y - margin,
            w: self.w + margin * T::TWO,
            h: self.h + margin * T::TWO,
        }
    }

    /// Convert this rectangle to a floating-point rectangle for precise calculations
    pub fn to_f32(self) -> Rect<f32> {
        Rect {
            x: self.x.to_f32(),
            y: self.y.to_f32(),
            w: self.w.to_f32(),
            h: self.h.to_f32(),
        }
    }

    pub fn to_i16(&self) -> Rect<i16> {
        Rect {
            x: self.x.to_f32() as i16,
            y: self.y.to_f32() as i16,
            w: self.w.to_f32() as i16,
            h: self.h.to_f32() as i16,
        }
    }

    /// Convert a floating-point rectangle to this rectangle's number type
    pub fn from_f32(rect: Rect<f32>) -> Self {
        Self {
            x: T::from_f32(rect.x),
            y: T::from_f32(rect.y),
            w: T::from_f32(rect.w),
            h: T::from_f32(rect.h),
        }
    }

    pub fn scale(&self, factor: T) -> Self {
        Rect {
            x: self.x / factor,
            y: self.y / factor,
            w: self.w / factor,
            h: self.h / factor,
        }
    }

    pub fn contains(&self, x: T, y: T) -> bool {
        x >= self.x && x <= self.x + self.w && y >= self.y && y <= self.y + self.h
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        let self_right = self.x + self.w;
        let self_bottom = self.y + self.h;
        let other_right = other.x + other.w;
        let other_bottom = other.y + other.h;

        self.x <= other_right
            && other.x <= self_right
            && self.y <= other_bottom
            && other.y <= self_bottom
    }
}

impl<T> Rect<T>
where
    T: Signed,
{
    pub fn sweep_x(self, delta: T) -> Rect<T> {
        if delta > T::ZERO {
            Rect {
                w: self.w + delta,
                ..self
            }
        } else {
            Rect {
                x: self.x + delta,
                w: self.w + (-delta),
                ..self
            }
        }
    }

    pub fn sweep_y(self, delta: T) -> Rect<T> {
        if delta > T::ZERO {
            Rect {
                h: self.h + delta,
                ..self
            }
        } else {
            Rect {
                y: self.y + delta,
                h: self.h + (-delta),
                ..self
            }
        }
    }
}

// Float-specific methods
impl<T> Rect<T>
where
    T: Float,
{
    /// Linear interpolation between two rectangles
    pub fn lerp(&self, other: &Self, t: T) -> Self {
        Self {
            x: self.x + (other.x - self.x) * t,
            y: self.y + (other.y - self.y) * t,
            w: self.w + (other.w - self.w) * t,
            h: self.h + (other.h - self.h) * t,
        }
    }

    pub fn floor(&self) -> Self {
        Rect {
            x: self.x.floor(),
            y: self.y.floor(),
            w: self.w.floor(),
            h: self.h.floor(),
        }
    }

    pub fn ceil(&self) -> Self {
        Rect {
            x: self.x.ceil(),
            y: self.y.ceil(),
            w: self.w.ceil(),
            h: self.h.ceil(),
        }
    }
}
