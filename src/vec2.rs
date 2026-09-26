use crate::{Num, SignedNum, num::Float};

mod ops;

/// A generic 2D vector.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T>
where
    T: Num,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn offset(self, delta_x: T, delta_y: T) -> Self {
        Self {
            x: self.x + delta_x,
            y: self.y + delta_y,
        }
    }

    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn from_f32(value: Vec2<f32>) -> Self {
        Self {
            x: T::from_f32(value.x),
            y: T::from_f32(value.y),
        }
    }

    pub fn to_f32(self) -> Vec2<f32> {
        Vec2 {
            x: self.x.to_f32(),
            y: self.y.to_f32(),
        }
    }
}

impl<T> Vec2<T>
where
    T: SignedNum,
{
    pub fn up() -> Self {
        Self {
            x: T::zero(),
            y: -T::one(),
        }
    }

    pub fn down() -> Self {
        Self {
            x: T::zero(),
            y: T::one(),
        }
    }

    pub fn left() -> Self {
        Self {
            x: -T::one(),
            y: T::zero(),
        }
    }

    pub fn right() -> Self {
        Self {
            x: T::one(),
            y: T::zero(),
        }
    }
}

impl<T> Vec2<T>
where
    T: Float,
{
    pub fn is_longer_than_zero(&self) -> bool {
        self.x.abs() > T::zero() || self.y.abs() > T::zero()
    }

    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }

    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }

    pub fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    pub fn len(&self) -> T {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let len = self.len();
        if len > T::epsilon() {
            Vec2 {
                x: self.x / len,
                y: self.y / len,
            }
        } else {
            *self
        }
    }

    pub fn average(&self, other: &Self) -> Self {
        Self {
            x: (self.x + other.x) / T::two(),
            y: (self.y + other.y) / T::two(),
        }
    }

    pub fn dot(&self, other: &Self) -> T {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn scale(&self, factor: T) -> Self {
        Vec2 {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    pub fn reflect(v: Self, n: Self) -> Self {
        let dot_product = v.x * n.x + v.y * n.y;
        Vec2 {
            x: v.x - T::two() * dot_product * n.x,
            y: v.y - T::two() * dot_product * n.y,
        }
    }
}

impl<T> Vec2<T>
where
    T: Float,
{
    pub fn angle_to(&self, other: &Self) -> T {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let angle = dy.atan2(dx);
        if angle < T::zero() {
            angle + T::two() * T::pi()
        } else {
            angle
        }
    }

    pub fn clamp_to_length(&mut self, max_length: T) {
        let current_length = self.len();
        if current_length > max_length && current_length > T::epsilon() {
            let normalized = self.normalize();
            self.x = normalized.x * max_length;
            self.y = normalized.y * max_length;
        }
    }

    pub fn rotate(&self, angle: T) -> Self {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();
        Vec2 {
            x: self.x * cos_theta - self.y * sin_theta,
            y: self.x * sin_theta + self.y * cos_theta,
        }
    }

    pub fn distance_to(&self, other: Self) -> T {
        let dist_x = other.x - self.x;
        let dist_y = other.y - self.y;
        ((dist_x * dist_x) + (dist_y * dist_y)).sqrt()
    }
}
