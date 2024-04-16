// This Vector2 implementation has been written from scratch (by me, CenTdemeern1) and is under a public domain license
// It is not considered part of any parent projects and may be modified or relicensed to suit another project's needs when added into a codebase

// Here this (original) version has been written to provide the features needed for the olc-util-geometry2d crate,
//  but features may be added or removed as the user sees fit, and it is not necessarily exclusive to said crate and may be copied to other projects and relicensed

// This implementation is quite generic anyway so hopefully I won't get in any legal hot water for not considering it a port of olcUTIL_Geometry2D's v_2d
// (because it isn't, this implementation is tailored toward Rust's feature set and standard practices)

// I am not a lawyer so if this is not how legal stuff works let me know and I can amend/change the above section

#![allow(dead_code)]
use std::ops::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

// Things that work for all vectors

impl<T> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Vector2 { x, y }
    }

    pub fn transpose(self) -> Self {
        vec2!(self.y, self.x)
    }
}

macro_rules! vec2 {
    ($x: expr, $y: expr) => {
        Vector2::new($x, $y)
    };
    ($x: expr) => {
        Vector2::new($x, $x)
    };
    () => {
        Vector2::default()
    };
}

pub(crate) use vec2;

macro_rules! vec2cast {
    ($vec: expr, $type: ty) => {
        vec2!($vec.x as $type, $vec.y as $type)
    };
}

pub(crate) use vec2cast;

// Equivalence traits

impl<T: PartialEq> PartialEq for Vector2<T> {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl<T: Eq> Eq for Vector2<T> {}

// Generate convenience vector math functions (vector * vector, etc)

macro_rules! impl_operator {
    ($trait: tt, $fn: ident, $trait_assign: tt, $fn_assign: ident) => {
        impl<T: $trait> $trait for Vector2<T> {
            type Output = Vector2<<T as $trait>::Output>;

            fn $fn(self, rhs: Self) -> Self::Output {
                vec2!($trait::$fn(self.x, rhs.x), $trait::$fn(self.y, rhs.y))
            }
        }

        impl<T: Copy + $trait> $trait<T> for Vector2<T> {
            type Output = Vector2<<T as $trait>::Output>;

            fn $fn(self, rhs: T) -> Self::Output {
                vec2!($trait::$fn(self.x, rhs), $trait::$fn(self.y, rhs))
            }
        }

        impl<T: $trait_assign> $trait_assign for Vector2<T> {
            fn $fn_assign(&mut self, rhs: Self) {
                $trait_assign::$fn_assign(&mut self.x, rhs.x);
                $trait_assign::$fn_assign(&mut self.y, rhs.y);
            }
        }

        impl<T: Copy + $trait_assign> $trait_assign<T> for Vector2<T> {
            fn $fn_assign(&mut self, rhs: T) {
                $trait_assign::$fn_assign(&mut self.x, rhs);
                $trait_assign::$fn_assign(&mut self.y, rhs);
            }
        }
    };
}

impl_operator!(Add, add, AddAssign, add_assign);
impl_operator!(Sub, sub, SubAssign, sub_assign);
impl_operator!(Mul, mul, MulAssign, mul_assign);
impl_operator!(Div, div, DivAssign, div_assign);

impl<T: Neg> Neg for Vector2<T> {
    type Output = Vector2<<T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        vec2!(-self.x, -self.y)
    }
}

// Square root trait

pub trait Sqrt {
    type Output;

    fn sqrt(self) -> Self::Output;
}

macro_rules! impl_sqrt {
    ($type: ty) => {
        impl Sqrt for $type {
            type Output = Self;

            fn sqrt(self) -> Self::Output {
                self.sqrt()
            }
        }
    };
}

impl_sqrt!(f32);
impl_sqrt!(f64);

impl<T: Sqrt> Sqrt for Vector2<T> {
    type Output = Vector2<<T as Sqrt>::Output>;

    fn sqrt(self) -> Self::Output {
        vec2!(self.x.sqrt(), self.y.sqrt())
    }
}

// Rounding trait

pub trait Round {
    type Output;

    fn floor(self) -> Self::Output;
    fn round(self) -> Self::Output;
    fn ceil(self) -> Self::Output;
}

macro_rules! impl_round {
    ($type: ty) => {
        impl Round for $type {
            type Output = Self;

            fn floor(self) -> Self::Output {
                self.floor()
            }

            fn round(self) -> Self::Output {
                self.round()
            }

            fn ceil(self) -> Self::Output {
                self.ceil()
            }
        }
    };
}

impl_round!(f32);
impl_round!(f64);

impl<T: Round> Round for Vector2<T> {
    type Output = Vector2<<T as Round>::Output>;

    fn floor(self) -> Self::Output {
        vec2!(self.x.floor(), self.y.floor())
    }

    fn round(self) -> Self::Output {
        vec2!(self.x.round(), self.y.round())
    }

    fn ceil(self) -> Self::Output {
        vec2!(self.x.ceil(), self.y.ceil())
    }
}

// Absolute value trait

pub trait Abs {
    type Output;

    fn abs(self) -> Self::Output;
}

macro_rules! impl_abs {
    ($type: ty) => {
        impl Abs for $type {
            type Output = Self;

            fn abs(self) -> Self::Output {
                self.abs()
            }
        }
    };
}

impl_abs!(i8);
impl_abs!(i16);
impl_abs!(i32);
impl_abs!(i64);
impl_abs!(i128);
impl_abs!(isize);
impl_abs!(f32);
impl_abs!(f64);

impl<T: Abs> Abs for Vector2<T> {
    type Output = Vector2<<T as Abs>::Output>;

    fn abs(self) -> Self::Output {
        vec2!(self.x.abs(), self.y.abs())
    }
}

// Radial math trait
// (I didn't want to make separate traits for all of these)

pub trait Radial {
    type Output;

    fn sin(self) -> Self::Output;
    fn cos(self) -> Self::Output;
    fn atan2(self, other: Self) -> Self::Output;
    // Thinking really hard about whether I should add sin_cos...
}

macro_rules! impl_radial {
    ($type: ty) => {
        impl Radial for $type {
            type Output = Self;

            fn sin(self) -> Self::Output {
                self.sin()
            }

            fn cos(self) -> Self::Output {
                self.cos()
            }

            fn atan2(self, other: Self) -> Self::Output {
                self.atan2(other)
            }
        }
    };
}

impl_radial!(f32);
impl_radial!(f64);

// Actual vector math

impl<T: Mul> Vector2<T> {
    pub fn area(self) -> <T as Mul>::Output {
        self.x * self.y
    }
}

impl<T: Copy + Mul> Vector2<T> {
    pub fn squared(self) -> Vector2<<T as Mul>::Output> {
        self * self
    }
}

impl<T: Add> Vector2<T> {
    pub fn sum(self) -> <T as Add>::Output {
        self.x + self.y
    }
}

impl<T> Vector2<T>
where
    T: Copy + Mul,
    <T as Mul>::Output: Add,
{
    pub fn mag2(self) -> <<T as Mul>::Output as Add>::Output {
        self.squared().sum()
    }
}

impl<T> Vector2<T>
where
    T: Copy + Mul,
    <T as Mul>::Output: Add,
    <<T as Mul>::Output as Add>::Output: Sqrt,
{
    pub fn mag(self) -> <<<T as Mul>::Output as Add>::Output as Sqrt>::Output {
        self.squared().sum().sqrt()
    }
}

impl<T> Vector2<T>
where
    T: Copy + Mul,
    Vector2<T>: Div<<<<T as Mul>::Output as Add>::Output as Sqrt>::Output>,
    <T as Mul>::Output: Add,
    <<T as Mul>::Output as Add>::Output: Sqrt,
{
    pub fn norm(
        self,
    ) -> <Vector2<T> as Div<<<<T as Mul>::Output as Add>::Output as Sqrt>::Output>>::Output {
        self / self.mag()
    }
}

impl<T> Vector2<T>
where
    T: Neg<Output = T>,
{
    pub fn perp(self) -> Self {
        vec2!(-self.y, self.x)
    }
}

macro_rules! impl_ord_fn {
    ($fn: ident) => {
        pub fn $fn(self, other: Self) -> Self {
            vec2!(Ord::$fn(self.x, other.x), Ord::$fn(self.y, other.y))
        }
    };
}

impl<T: Ord> Vector2<T> {
    impl_ord_fn!(min);
    impl_ord_fn!(max);

    pub fn clamp(self, min: Self, max: Self) -> Self {
        vec2!(
            Ord::clamp(self.x, min.x, max.x),
            Ord::clamp(self.y, min.y, max.y)
        )
    }
}

impl<T> Vector2<T>
where
    T: Mul,
    <T as Mul>::Output: Add,
{
    pub fn dot(self, rhs: Vector2<T>) -> <<T as Mul>::Output as Add>::Output {
        (self * rhs).sum()
    }
}

impl<T> Vector2<T>
where
    T: Mul,
    <T as Mul>::Output: Sub,
{
    pub fn cross(self, rhs: Vector2<T>) -> <<T as Mul>::Output as Sub>::Output {
        let Vector2 { x, y } = self * rhs.transpose();
        x - y
    }
}

impl<T> Vector2<T>
where
    T: Copy + Radial,
    <T as Radial>::Output: Mul<T>,
{
    pub fn cart(self) -> Vector2<<<T as Radial>::Output as Mul<T>>::Output> {
        vec2!(self.y.cos() * self.x, self.y.sin() * self.x)
    }
}

impl<T, O> Vector2<T>
where
    T: Copy + Mul + Radial<Output = O>,
    <T as Mul>::Output: Add,
    <<T as Mul>::Output as Add>::Output: Sqrt<Output = O>,
{
    pub fn polar(self) -> Vector2<O> {
        vec2!(self.mag(), self.y.atan2(self.x))
    }
}

impl<T> Vector2<T>
where
    T: Copy + Sub,
    Vector2<<T as Sub>::Output>: Mul<T>,
    <Vector2<<T as Sub>::Output> as Mul<T>>::Output: Add<Self>,
{
    pub fn lerp(
        self,
        to: Self,
        by: T,
    ) -> <<Vector2<<T as Sub>::Output> as Mul<T>>::Output as Add<Vector2<T>>>::Output {
        (to - self) * by + self
    }
}

impl<T> Vector2<T>
where
    T: Copy + Mul + From<u8>,
    <T as Mul>::Output: Add,
    <<T as Mul>::Output as Add>::Output: Mul<Vector2<T>>,
    <<<T as Mul>::Output as Add>::Output as Mul<Vector2<T>>>::Output: Mul<T>,
    Vector2<T>:
        Sub<<<<<T as Mul>::Output as Add>::Output as Mul<Vector2<T>>>::Output as Mul<T>>::Output>,
{
    pub fn reflect(
        self,
        along: Self,
    ) -> <Vector2<T> as Sub<
        <<<<T as Mul>::Output as Add>::Output as Mul<Vector2<T>>>::Output as Mul<T>>::Output,
    >>::Output {
        self - self.dot(along) * along * (T::from(2))
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_vector2() {
        // Standard integer vector
        let vector = vec2!(5, 9);
        assert_eq!(vector.x, 5);
        assert_eq!(vector.y, 9);

        // Macro to assign both X and Y to the same value
        let vector = vec2!(7);
        assert_eq!(vector.x, 7);
        assert_eq!(vector.y, 7);

        // Macro to get a default vector of the given type
        assert_eq!(vec2!(), vec2!(0));

        // I have no clue why you would do this, but hey, more power to you
        // This will probably satisfy exactly zero trait bounds so pretty much
        //  all of the methods Vector2 provides will be useless to you :D
        let vector = vec2!('a', 'b');
        assert_eq!(vector.x, 'a');
        assert_eq!(vector.y, 'b');
    }

    #[test]
    fn test_get_area() {
        assert_eq!(vec2!(5, 9).area(), 45);
    }

    #[test]
    fn test_mul() {
        assert_eq!(vec2!(5, 9) * vec2!(9, 5), vec2!(45));
        assert_eq!(vec2!(5, 9) * 9, vec2!(45, 81));
    }

    #[test]
    fn test_norm() {
        let vector = vec2!(0.697901_f32, 0.97343_f32).norm();
        // Account for floating point inaccuracy
        assert!((vector - vec2!(0.582671, 0.812708)).abs().sum() < 0.0001);
    }

    #[test]
    fn test_radial() {
        let vector = vec2!(0.697901_f32, 0.97343_f32);
        assert!((vector.polar().cart() - vector).abs().sum() < 0.0001);
    }

    #[test]
    fn test_lerp() {
        let vector = vec2!(2., -10.).lerp(vec2!(10.), 0.5);
        assert_eq!(vector, vec2!(6., 0.));
    }

    #[test]
    fn test_vec2cast() {
        assert_eq!(vec2!(20i16), vec2cast!(vec2!(20u8), i16))
    }
}
